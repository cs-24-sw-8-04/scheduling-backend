use derive_more::{Add, From, Mul};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Add, Mul, From, sqlx::Type)]
#[sqlx(transparent)]
pub struct TimeUnit(i64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Add, Mul, From, sqlx::Type)]
#[sqlx(transparent)]
pub struct Duration(TimeUnit);

impl From<i64> for Duration {
    fn from(value: i64) -> Self {
        let time_unit: TimeUnit = value.into();
        time_unit.into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: TimeUnit,
    pub end: TimeUnit,
}

impl TimeSlot {
    pub fn new(start: TimeUnit, end: TimeUnit) -> Self {
        TimeSlot { start, end }
    }
}
