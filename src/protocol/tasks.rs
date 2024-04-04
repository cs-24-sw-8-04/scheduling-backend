use serde::{Deserialize, Serialize};

use crate::data_model::time::{Milliseconds, Timespan};

#[derive(Deserialize, Serialize)]
pub struct CreateTaskRequest {
    pub timespan: Timespan,
    pub duration: Milliseconds,
    pub device_id: i64,
}
