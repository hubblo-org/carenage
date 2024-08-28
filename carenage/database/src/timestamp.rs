use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Timestamp {
    UnixTimestamp(Option<u64>),
    ISO8601Timestamp(Option<DateTime<Utc>>),
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Timestamp::UnixTimestamp(value) => {
                write!(f, "{}", value.expect("Unable to parse Unix Timestamp"))
            }
            Timestamp::ISO8601Timestamp(value) => {
                write!(f, "{}", value.expect("Unable to parse ISO 8601"))
            }
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

    pub fn parse_str(timestamp_str: String, unix: bool) -> Timestamp {
        match unix {
            true => Timestamp::UnixTimestamp(Some(
                timestamp_str
                    .parse::<u64>()
                    .expect("The string should be parsable to convert it to UNIX timestamp."),
            )),
            false => Timestamp::ISO8601Timestamp(Some(
                timestamp_str
                    .parse()
                    .expect("The string should be parsable to convert it to ISO8601 timestamp."),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn it_parses_a_string_to_return_an_unix_timestamp() {
        let unix_timestamp_str = "1724833101".to_string();
        let parsed_string = Timestamp::parse_str(unix_timestamp_str, true);
        assert_eq!(parsed_string, Timestamp::UnixTimestamp(Some(1724833101)));
    }

    #[test]
    fn it_parses_a_string_to_return_an_iso8601_timestamp() {
        let now_iso8601 = Utc::now();
        let iso8601_timestamp_str = now_iso8601.to_string(); 
        let parsed_string = Timestamp::parse_str(iso8601_timestamp_str, false);
        assert_eq!(parsed_string, Timestamp::ISO8601Timestamp(Some(now_iso8601)));
    }

    #[test]
    #[should_panic]
    fn it_fails_to_parse_a_string_as_unix_timestamp() {
        let bound_to_fail = "boundtofail".to_string();
        let _parsed_string = Timestamp::parse_str(bound_to_fail, true);
    }

    #[test]
    #[should_panic]
    fn it_fails_to_parse_a_string_as_iso8601_timestamp() {
        let bound_to_fail = "boundtofail".to_string();
        let _parsed_string = Timestamp::parse_str(bound_to_fail, false);
    }
}
