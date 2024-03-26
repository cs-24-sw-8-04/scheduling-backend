use serde::{Deserialize, Serialize};

use super::time::{Milliseconds, Timespan};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub timespan: Timespan,
    pub duration: Milliseconds,
    pub effect: f64,
}
