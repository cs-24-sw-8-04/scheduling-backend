use chrono::{DateTime, Local};
use uuid::Uuid;

struct Event {
    device_id: Uuid,
    event_id: Uuid,
    version_nr: i32,
    start_time: DateTime<Local>,
}
