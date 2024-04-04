use serde::{Deserialize, Serialize};

use super::time::DateTimeUtc;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub device_id: i64,
    pub version_nr: i64,
    pub start_time: DateTimeUtc,
}
