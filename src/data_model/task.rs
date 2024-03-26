use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Timespan {
    pub start: i64,
    pub end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub timespan: Timespan,
    pub duration: i64,
    pub effect: f64,
}
