use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::error::Error;
use tokio::net::TcpListener;

use dotenv::dotenv;

mod data_model;

use data_model::{task::Task, time::TimeSlot};

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
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[debug_handler]
async fn get_tasks(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = sqlx::query!(
        r#"
    SELECT id, timeslot_start, timeslot_end, duration, effect
    FROM Tasks
    "#
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    let my_tasks = tasks
        .iter()
        .map(|t| Task {
            id: t.id,
            time_slot: TimeSlot::new(t.timeslot_start.into(), t.timeslot_end.into()),
            duration: t.duration.into(),
            effect: t.effect,
        })
        .collect();

    Ok(Json(my_tasks))
}

#[debug_handler]
async fn create_task(
    State(pool): State<SqlitePool>,
    Json(mut task): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let id = sqlx::query_scalar!(
        r#"
    INSERT INTO Tasks (timeslot_start, timeslot_end, duration, effect)
    VALUES (?, ?, ?, ?)
    RETURNING id
    "#,
        task.time_slot.start,
        task.time_slot.end,
        task.duration,
        task.effect
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;

    task.id = id;

    Ok(Json(task))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
