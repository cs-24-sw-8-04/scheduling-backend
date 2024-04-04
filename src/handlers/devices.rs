use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{
    data_model::device::Device, extractors::auth::Authentication, handlers::util::internal_error,
    protocol::devices::CreateDeviceRequest,
};

#[debug_handler]
pub async fn get_all_smart_devices(
    State(pool): State<SqlitePool>,
    Authentication(account_id): Authentication,
) -> Result<Json<Vec<Device>>, (StatusCode, String)> {
    let devices = sqlx::query!(
        r#"
			SELECT id, effect, account_id
			FROM Devices
			WHERE account_id = ?
		"#,
        account_id
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(
        devices
            .iter()
            .map(|d| Device {
                id: d.id,
                effect: d.effect,
                account_id: d.account_id,
            })
            .collect(),
    ))
}

#[debug_handler]
pub async fn create_smart_device(
    State(pool): State<SqlitePool>,
    Authentication(account_id): Authentication,
    Json(create_device_request): Json<CreateDeviceRequest>,
) -> Result<Json<Device>, (StatusCode, String)> {
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO Devices (effect, account_id)
            VALUES (?, ?)
            RETURNING id
        "#,
        create_device_request.effect,
        account_id
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;

    let device = Device {
        id,
        effect: create_device_request.effect,
        account_id,
    };

    Ok(Json(device))
}

#[debug_handler]
pub async fn delete_smart_device(
    State(pool): State<SqlitePool>,
    Authentication(account_id): Authentication,
    Json(device): Json<Device>,
) -> Result<(), (StatusCode, String)> {
    sqlx::query!(
        r#"
        DELETE FROM Devices
        WHERE id == ? AND account_id == ?
        "#,
        device.id,
        account_id
    )
    .execute(&pool)
    .await
    .map_err(internal_error)?;

    Ok(())
}
