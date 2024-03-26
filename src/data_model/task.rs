use::uuid::Uuid;
use::chrono::{DateTime, Local};
struct TimeInteval {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
}

struct Task {
    device_id: Uuid,
    effect: i16,
    duration: i32,
    interval: TimeInteval,
}

pub fn hello() {
    println!("Hello World");
}