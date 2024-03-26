use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Device {
    id: i64,
    effect: f64,
}
