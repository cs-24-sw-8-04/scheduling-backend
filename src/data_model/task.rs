use serde::{Deserialize, Serialize};

use crate::data_model::time::{Duration, TimeSlot};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub time_slot: TimeSlot,
    pub duration: Duration,
    pub effect: f64,
}
