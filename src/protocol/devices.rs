use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateDeviceRequest {
    pub effect: f64,
}
