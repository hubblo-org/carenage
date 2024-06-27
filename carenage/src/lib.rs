use reqwest::blocking::{Client, Response};

fn query_boagent(
    boagent_url: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    location: String,
    lifetime: u8,
) -> Result<Response, String> {
    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);
    let query_parameters = vec![
        ("start_time", start_time.expect("unable to parse timestamp").to_string()),
        ("end_time", end_time.expect("unable to parse timestamp").to_string()),
        ("verbose", "true".to_string()),
        ("location", location),
        ("measure_power", "true".to_string()),
        ("lifetime", lifetime.to_string()),
        ("fetch_hardware", "true".to_string()),
    ];
    let response = client
        .get(base_url)
        .query(&query_parameters)
        .send()
        .expect("Failure to execute request.");

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

    #[test]
    fn it_queries_boagent_with_success_with_needed_query_paramaters() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 8000,
            ..Default::default()
        };

        let mut boagent_server = Server::new_with_opts(opts);

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
            "http://127.0.0.1:8000".to_string(),
            Some(now_timestamp_minus_one_minute),
            Some(now_timestamp),
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

        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 8001,
            ..Default::default()
        };

        let mut boagent_server = Server::new_with_opts(opts);

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
            "http://127.0.0.1:8001".to_string(),
            Some(now_timestamp_minus_one_minute),
            Some(now_timestamp),
            "FRA".to_string(),
            5,
        );

        assert_eq!(response.is_err(), true);
    }
}
