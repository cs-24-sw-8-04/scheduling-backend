use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{
	extractors::auth::Authentication,
	data_model::device::Device,
	handlers::util::internal_error
};

#[debug_handler]
pub async fn get_all_smart_devices(
	State(pool): State<SqlitePool>,
	Authentication(account_id): Authentication
) -> Result<Json<Vec<Device>>, (StatusCode, String)>{
	let devices = sqlx::query!(
		r#"
			SELECT device_id, effect
			FROM Devices
			WHERE account_id = ?
		"#,
		account_id
	)
	.fetch_all(&pool)
	.await
	.map_err(internal_error)?;

	Ok(Json(devices
		.iter()
		.map(|d| Device {
			id: d.device_id,
			effect: d.effect,
		})
		.collect()))
}