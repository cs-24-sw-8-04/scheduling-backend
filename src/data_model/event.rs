use serde::{Deserialize, Serialize};

use super::time::DateTimeUtc;

#[derive(Serialize, Deserialize)]
struct Event {
    id: i64,
    device_id: i64,
    version_nr: i64,
    start_time: DateTimeUtc,
}
