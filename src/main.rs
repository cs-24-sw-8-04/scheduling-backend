mod data_model;
mod extractors;
mod handlers;
mod protocol;

use std::error::Error;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::net::TcpListener;

use handlers::{accounts::*, devices::*, tasks::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let db_connection_string = std::env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .connect(&db_connection_string)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    let app = app(pool);

    axum::serve(listener, app).await?;

    Ok(())
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/tasks/all", get(get_tasks))
        .route("/tasks/create", post(create_task))
        .route("/task/delete", post(delete_task))
        .route("/device/all", get(get_all_smart_devices))
        .route("/device/create", post(create_smart_device))
        .route("/device/delete", post(delete_smart_device))
        .route("/accounts/register", post(register_account))
        .route("/accounts/login", post(login_to_account))
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use crate::data_model::{task::Task, time::Timespan};

    use self::{
        data_model::device::Device,
        extractors::auth::AuthToken,
        protocol::{
            accounts::{RegisterOrLoginRequest, RegisterOrLoginResponse},
            devices::CreateDeviceRequest,
        },
    };

    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
        routing::RouterIntoService,
    };
    use chrono::{Days, Utc};
    use http_body_util::BodyExt;
    use tower::{Service, ServiceExt};
    use uuid::Uuid;

    async fn test_app() -> Router {
        let db_connection_string = "sqlite::memory:";

        let pool = SqlitePoolOptions::new()
            .connect(db_connection_string)
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        app(pool)
    }

    async fn get_account(app: &mut RouterIntoService<Body>) -> AuthToken {
        let request = Request::builder()
            .method(Method::POST)
            .uri("/accounts/register")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&RegisterOrLoginRequest {
                    username: "test_user".to_string(),
                    password: "test_password".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let response: RegisterOrLoginResponse = serde_json::from_slice(&body).unwrap();

        response.auth_token
    }

    async fn generate_device(app: &mut RouterIntoService<Body>, auth_token: String) -> Device {
        let request = Request::builder()
            .method(Method::POST)
            .uri("/device/create")
            .header("Content-Type", "application/json")
            .header("X-Auth-Token", auth_token.clone())
            .body(Body::from(
                serde_json::to_vec(&CreateDeviceRequest { effect: 1000.0 }).unwrap(),
            ))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        println!("{:?}", response);

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let device: Device = serde_json::from_slice(&body).unwrap();

        device
    }

    fn auth_token_to_uuid(auth_token: AuthToken) -> String {
        let auth_token_json = serde_json::to_string(&auth_token).unwrap();
        let uuid: Uuid = serde_json::from_str(&auth_token_json).unwrap();
        uuid.hyphenated().to_string()
    }

    #[tokio::test]
    async fn register_account() {
        let mut app = test_app().await.into_service();

        get_account(&mut app).await;
    }

    #[tokio::test]
    async fn login_to_account() {
        let mut app = test_app().await.into_service();

        // Registers an account
        get_account(&mut app).await;

        // Login to account
        let request = Request::builder()
            .method(Method::POST)
            .uri("/accounts/login")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&RegisterOrLoginRequest {
                    username: "test_user".to_string(),
                    password: "test_password".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let _response: RegisterOrLoginResponse = serde_json::from_slice(&body).unwrap();
    }

    #[tokio::test]
    async fn create_task() {
        let mut app = test_app().await.into_service();

        // Registers an account
        let auth_token = get_account(&mut app).await;
        let auth_token = auth_token_to_uuid(auth_token);
        let device = generate_device(&mut app, auth_token.clone()).await;

        let request = Request::builder()
            .method(Method::POST)
            .uri("/tasks/create")
            .header("Content-Type", "application/json")
            .header("X-Auth-Token", auth_token.clone())
            .body(Body::from(
                serde_json::to_vec(&Task {
                    id: -1,
                    timespan: Timespan::new(
                        Utc::now(),
                        Utc::now().checked_add_days(Days::new(1)).unwrap(),
                    ),
                    duration: 3600.into(),
                    device_id: device.id,
                })
                .unwrap(),
            ))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let response: Task = serde_json::from_slice(&body).unwrap();

        assert_ne!(response.id, -1);
        assert_eq!(response.duration, 3600.into());
    }

    #[tokio::test]
    async fn get_all_tasks() {
        let mut app = test_app().await.into_service();

        // Registers an account
        let auth_token = get_account(&mut app).await;
        let auth_token = auth_token_to_uuid(auth_token);
        let device = generate_device(&mut app, auth_token.clone()).await;

        let request = Request::builder()
            .method(Method::POST)
            .uri("/tasks/create")
            .header("Content-Type", "application/json")
            .header("X-Auth-Token", auth_token.clone())
            .body(Body::from(
                serde_json::to_vec(&Task {
                    id: -1,
                    timespan: Timespan::new(
                        Utc::now(),
                        Utc::now().checked_add_days(Days::new(1)).unwrap(),
                    ),
                    duration: 3600.into(),
                    device_id: device.id,
                })
                .unwrap(),
            ))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let created_task: Task = serde_json::from_slice(&body).unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/tasks/all")
            .header("Content-Type", "application/json")
            .header("X-Auth-Token", auth_token)
            .body(Body::empty())
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            let body = String::from_utf8_lossy(&body);
            panic!("{}", body);
        }

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let all_tasks: Vec<Task> = serde_json::from_slice(&body).unwrap();

        assert_eq!(all_tasks.first().unwrap(), &created_task);
    }
}
