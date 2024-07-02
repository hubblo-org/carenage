use reqwest::blocking::{Client, Response};
use chrono::{DateTime, Utc};

enum Timestamp {
    UnixTimestamp(Option<u64>),
    ISO8601Timestamp(DateTime<Utc>),
}

fn query_boagent(
    boagent_url: String,
    start_time: Timestamp,
    end_time: Timestamp,
    location: String,
    lifetime: u8,
) -> Result<Response, String> {
    
    let mut query_parameters = vec![];

    match start_time {
        Timestamp::UnixTimestamp(start_time) => {query_parameters.push(("start_time", start_time.unwrap_or(0).to_string()))},
        Timestamp::ISO8601Timestamp(start_time) => {query_parameters.push(("start_time", format!("{:?}", start_time)))}
    }
    match end_time {
        Timestamp::UnixTimestamp(end_time) => {query_parameters.push(("end_time", end_time.unwrap_or(0).to_string()))},
        Timestamp::ISO8601Timestamp(end_time) => {query_parameters.push(("end_time", format!("{:?}", end_time)))}
    }
    query_parameters.push(("verbose", "true".to_string()));
    query_parameters.push(("location", location));
    query_parameters.push(("measure_power", "true".to_string()));
    query_parameters.push(("lifetime", lifetime.to_string()));
    query_parameters.push(("fetch_hardware", "true".to_string()));
    
    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);

    let response = client
        .get(base_url)
        .query(&query_parameters)
        .send()
        .unwrap();

    match response.status().as_u16() {
        200 => Ok(response),
        _ => Err("Error from Boagent.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use std::time::SystemTime;
    use chrono::{Duration, Utc};

    #[test]
    fn it_queries_boagent_with_success_with_needed_query_paramaters() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={:?}", now_timestamp_minus_one_minute).into()),
                Matcher::Regex(format!("end_time={:?}", now_timestamp).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            "FRA".to_string(),
            5,
        )
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn it_queries_boagent_with_error_if_500_http_code_received_from_boagent() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={:?}", now_timestamp_minus_one_minute).into()),
                Matcher::Regex(format!("end_time={:?}", now_timestamp).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(500)
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            "FRA".to_string(),
            5,
        );

        assert_eq!(response.is_err(), true);
    }
    
    #[test]
    fn it_queries_boagent_with_success_with_unspecified_timestamps() {

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex("start_time=0".into()),
                Matcher::Regex("end_time=0".into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(None),
            Timestamp::UnixTimestamp(None),
            "FRA".to_string(),
            5,
        ).unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn it_queries_boagent_with_success_with_iso_8601_timestamps() {

        let now_timestamp = Utc::now(); 
        let now_timestamp_minus_one_minute = now_timestamp - Duration::minutes(1);

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start_time".into(), format!("{:?}", now_timestamp_minus_one_minute)),
                Matcher::UrlEncoded("end_time".into(), format!("{:?}", now_timestamp)),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(
            url,
            Timestamp::ISO8601Timestamp(now_timestamp_minus_one_minute),
            Timestamp::ISO8601Timestamp(now_timestamp),
            "FRA".to_string(),
            5,
        ).unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }
}
