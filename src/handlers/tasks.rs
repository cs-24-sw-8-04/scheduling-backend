use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{
    data_model::{task::Task, time::Timespan},
    extractors::auth::Authentication,
    handlers::util::internal_error,
};

#[debug_handler]
pub async fn get_tasks(
    State(pool): State<SqlitePool>,
    Authentication(account_id): Authentication,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = sqlx::query!(
        r#"
        SELECT id, timespan_start, timespan_end, duration, effect
        FROM Tasks
        WHERE account_id = ?
        "#,
        account_id
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    let my_tasks = tasks
        .iter()
        .map(|t| Task {
            id: t.id,
            timespan: Timespan::new_from_naive(t.timespan_start, t.timespan_end),
            duration: t.duration.into(),
            effect: t.effect,
        })
        .collect();

    Ok(Json(my_tasks))
}

#[debug_handler]
pub async fn create_task(
    State(pool): State<SqlitePool>,
    Authentication(account_id): Authentication,
    Json(mut task): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO Tasks (timespan_start, timespan_end, duration, effect, account_id)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#,
        task.timespan.start,
        task.timespan.end,
        task.duration,
        task.effect,
        account_id
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;

    task.id = id;

    Ok(Json(task))
}
