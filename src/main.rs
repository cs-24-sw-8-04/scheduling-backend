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
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;

use handlers::{accounts::*, tasks::*, devices::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let db_connection_string = std::env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .connect(&db_connection_string)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(get_tasks))
        .route("/create", post(create_task))
        .route("/task/delete", post(delete_task))
        .route("/device/all", get(get_all_smart_devices))
        .route("/accounts/register", post(register_account))
        .route("/accounts/login", post(login_to_account))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
