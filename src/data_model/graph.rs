use super::time::Timespan;
use chrono::Duration;
pub struct DescreteGraph {
    values: Vec<(i32, Timespan)>,
    time_delta: Duration,
}

impl DescreteGraph {
    pub fn get_values(&self) -> &Vec<(i32, Timespan)> {
        &self.values
    }
}
