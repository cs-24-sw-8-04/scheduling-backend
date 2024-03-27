use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{
    extractors::auth::create_auth_token,
    handlers::util::internal_error,
    protocol::accounts::{RegisterOrLoginRequest, RegisterOrLoginResponse},
};

#[debug_handler]
pub async fn register_account(
    State(pool): State<SqlitePool>,
    Json(register_request): Json<RegisterOrLoginRequest>,
) -> Result<Json<RegisterOrLoginResponse>, (StatusCode, String)> {
    let password = register_request.password;
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password_bytes, &salt)
        .map_err(internal_error)?
        .to_string();

    let account_id = sqlx::query_scalar!(
        r#"
        INSERT INTO Accounts (username, password_hash)
        VALUES (?, ?)
        RETURNING id
        "#,
        register_request.username,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;

    let auth_token = create_auth_token(account_id, &pool)
        .await
        .map_err(internal_error)?;

    Ok(Json(RegisterOrLoginResponse { auth_token }))
}

#[debug_handler]
pub async fn login_to_account(
    State(pool): State<SqlitePool>,
    Json(login_request): Json<RegisterOrLoginRequest>,
) -> Result<Json<RegisterOrLoginResponse>, (StatusCode, String)> {
    let account = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM Accounts
        WHERE username = ?
        "#,
        login_request.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(internal_error)?;

    let account = account.ok_or((
        StatusCode::NOT_FOUND,
        "No account with username exists".to_string(),
    ))?;

    let password_hash = PasswordHash::new(&account.password_hash).map_err(internal_error)?;

    Argon2::default()
        .verify_password(login_request.password.as_bytes(), &password_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid password".to_string()))?;

    let auth_token = create_auth_token(account.id, &pool)
        .await
        .map_err(internal_error)?;
    Ok(Json(RegisterOrLoginResponse { auth_token }))
}
