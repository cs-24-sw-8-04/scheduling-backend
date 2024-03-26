use uuid::Uuid;
use::chrono::{DateTime, Local};

struct Event {
    device_id: Uuid,
    event_id: Uuid,
    version_nr: i32,
    start_time: DateTime<Local>,
}