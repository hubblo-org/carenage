use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Debug)]
pub enum Timestamp {
    UnixTimestamp(Option<u64>),
    ISO8601Timestamp(Option<DateTime<Utc>>),
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Timestamp::UnixTimestamp(value) => write!(
                f,
                "{}",
                value.expect("Unable to parse Unix Timestamp").to_string()
            ),
            Timestamp::ISO8601Timestamp(value) => write!(
                f,
                "{}",
                value.expect("Unable to parse ISO 8601").to_string()
            ),
        }
    }
}

impl Timestamp {
    pub fn new(unix: bool) -> Timestamp {
        match unix {
            true => Timestamp::UnixTimestamp(Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )),
            false => Timestamp::ISO8601Timestamp(Some(Utc::now())),
        }
    }
}
