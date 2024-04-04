use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: i64,
    pub effect: f64,
    pub account_id: i64,
}
