use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Copy, Clone)]
pub enum UnixFlag {
    Set,
    Unset,
}

impl From<bool> for UnixFlag {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Set,
            false => Self::Unset,
        }
    }
}
impl Display for UnixFlag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            UnixFlag::Set => {
                write!(f, "UnixFlag is set.")
            }
            UnixFlag::Unset => {
                write!(f, "UnixFlag is unset")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Timestamp {
    Unix(Option<u64>),
    ISO8601(Option<DateTime<Local>>),
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Timestamp::Unix(value) => {
                write!(f, "{}", value.expect("Unable to parse Unix Timestamp"))
            }
            Timestamp::ISO8601(value) => {
                write!(f, "{}", value.expect("Unable to parse ISO 8601"))
            }
        }
    }
}

impl Timestamp {
    pub fn new(unix_flag: UnixFlag) -> Timestamp {
        match unix_flag {
            UnixFlag::Set => Timestamp::Unix(Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )),
            UnixFlag::Unset => Timestamp::ISO8601(Some(Local::now())),
        }
    }

    pub fn parse_str(timestamp_str: String, unix_flag: UnixFlag) -> Timestamp {
        match unix_flag {
            UnixFlag::Set => Timestamp::Unix(Some(
                timestamp_str
                    .parse::<u64>()
                    .expect("The string should be parsable to convert it to UNIX timestamp."),
            )),
            UnixFlag::Unset => Timestamp::ISO8601(Some(
                timestamp_str
                    .parse()
                    .expect("The string should be parsable to convert it to ISO8601 timestamp."),
            )),
        }
    }

    pub fn as_query_parameter(&self) -> String {
        match self {
            Timestamp::Unix(value) => value.unwrap_or(0).to_string(),
            Timestamp::ISO8601(value) => value.unwrap_or(Local::now()).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn it_parses_a_string_to_return_an_unix_timestamp() {
        let unix_timestamp_str = "1724833101".to_string();
        let parsed_string = Timestamp::parse_str(unix_timestamp_str, UnixFlag::Set);
        assert_eq!(parsed_string, Timestamp::Unix(Some(1724833101)));
    }

    #[test]
    fn it_parses_a_string_to_return_an_iso8601_timestamp() {
        let now_iso8601 = Local::now();
        let iso8601_timestamp_str = now_iso8601.to_string();
        let parsed_string = Timestamp::parse_str(iso8601_timestamp_str, UnixFlag::Unset);
        assert_eq!(
            parsed_string,
            Timestamp::ISO8601(Some(now_iso8601))
        );
    }

    #[test]
    #[should_panic]
    fn it_fails_to_parse_a_string_as_unix_timestamp() {
        let bound_to_fail = "boundtofail".to_string();
        let _parsed_string = Timestamp::parse_str(bound_to_fail, UnixFlag::Set);
    }

    #[test]
    #[should_panic]
    fn it_fails_to_parse_a_string_as_iso8601_timestamp() {
        let bound_to_fail = "boundtofail".to_string();
        let _parsed_string = Timestamp::parse_str(bound_to_fail, UnixFlag::Unset);
    }
}
