use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub type DateTimeUtc = DateTime<Utc>;

#[derive(Serialize, Deserialize, sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct Milliseconds(i64);

impl Into<Duration> for Milliseconds {
    fn into(self) -> Duration {
        Duration::try_milliseconds(self.0).unwrap()
    }
}

impl From<Duration> for Milliseconds {
    fn from(value: Duration) -> Self {
        Milliseconds(value.num_milliseconds())
    }
}

impl From<i64> for Milliseconds {
    fn from(value: i64) -> Self {
        Milliseconds(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Timespan {
    pub start: DateTimeUtc,
    pub end: DateTimeUtc,
}

impl Timespan {
    pub fn new(start: DateTimeUtc, end: DateTimeUtc) -> Self {
        Timespan { start, end }
    }

    pub fn new_from_naive(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        fn to_utc(date_time: NaiveDateTime) -> DateTimeUtc {
            Utc::from_utc_datetime(&Utc, &date_time)
        }

        Timespan {
            start: to_utc(start),
            end: to_utc(end),
        }
    }
}
