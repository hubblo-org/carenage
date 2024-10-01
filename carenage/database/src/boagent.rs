use crate::timestamp::Timestamp;
use chrono::Local;
use dotenv::{from_path, var};
use reqwest::{Client, Response};
use serde_json::{Error, Value};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Clone, Copy)]
pub enum HardwareData {
    Inspect,
    Ignore,
}

impl Display for HardwareData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            HardwareData::Inspect => {
                write!(f, "true")
            }
            HardwareData::Ignore => {
                write!(f, "false")
            }
        }
    }
}

pub struct Config {
    pub boagent_url: String,
    pub database_url: String,
    pub location: String,
    pub lifetime: i16,
    pub device_name: String,
    pub project_name: String,
}

impl Config {
    pub fn check_configuration(config_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
        let _load_config = from_path(config_path);
        let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data.");
        let project_name = var("PROJECT_NAME").expect("PROJECT_NAME environment variable is absent. It is needed to refer to the project in collected data.");
        let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts.");
        let lifetime: i16 = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.").parse().expect("Failed to parse lifetime value.");
        let device_name = var("DEVICE").unwrap_or("unknown".to_string());
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable is absent.");
        Ok(Config {
            boagent_url,
            project_name,
            location,
            lifetime,
            device_name,
            database_url,
        })
    }
}

pub async fn query_boagent(
    boagent_url: String,
    start_time: Timestamp,
    end_time: Timestamp,
    fetch_hardware: HardwareData,
    location: String,
    lifetime: i16,
) -> Result<Response, reqwest::Error> {
    let query_parameters = vec![
        ("start_time", start_time.as_query_parameter()),
        ("end_time", end_time.as_query_parameter()),
        ("verbose", "true".to_string()),
        ("location", location.to_string()),
        ("measure_power", "true".to_string()),
        ("lifetime", lifetime.to_string()),
        ("fetch_hardware", fetch_hardware.to_string()),
    ];

    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);

    client.get(base_url).query(&query_parameters).send().await
}

pub async fn deserialize_boagent_json(boagent_response: Response) -> Result<Value, Error> {
    let deserialized_boagent_json = serde_json::from_value(boagent_response.json().await.unwrap())?;

    Ok(deserialized_boagent_json)
}

pub fn get_processes_ids(deseriliazed_boagent_response: Value) -> Result<Vec<u64>, Error> {
    // Need to get last measured processes by Scaphandre: processes during the execution of
    // carenaged and the associated Scaphandre measurements might change, depending on the
    // configuration set for Scaphandre (ten most energy intensive processes, or something else).
    // By getting the last item in raw_data from Scaphandre, those processes will be the last
    // measured by Scaphandre.
    let last_timestamp = deseriliazed_boagent_response["raw_data"]["power_data"]["raw_data"]
        .as_array()
        .expect("Data from Scaphandre should be parsable.")
        .last()
        .unwrap();
    let processes = last_timestamp["consumers"]
        .as_array()
        .expect("Processes should be parsable from Scaphandre.")
        .iter();

    let processes_ids = processes
        .map(|process| {
            process
                .get("pid")
                .expect("Consumer should have a pid key.")
                .as_u64()
                .expect("Process ID returned from Scaphandre should be parsable.")
        })
        .collect();
    Ok(processes_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};
    use mockito::{Matcher, Server};
    use std::time::SystemTime;

    #[sqlx::test]
    async fn it_queries_boagent_with_success_with_needed_query_paramaters() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={}", now_timestamp_minus_one_minute)),
                Matcher::Regex(format!("end_time={}", now_timestamp)),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        mock.assert_async().await;

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    fn it_queries_boagent_with_success_with_unspecified_timestamps() {
        let mut boagent_server = Server::new_async().await;

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
            .create_async()
            .await;

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(None),
            Timestamp::UnixTimestamp(None),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    fn it_queries_boagent_with_success_with_iso_8601_timestamps() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    async fn it_sends_an_error_when_it_fails_to_send_a_request_to_boagent() {
        let url = "http://url.will.fail".to_string();

        let response = query_boagent(
            url,
            Timestamp::ISO8601Timestamp(None),
            Timestamp::ISO8601Timestamp(None),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await;

        assert!(response.is_err());
    }

    #[sqlx::test]
    async fn it_deserializes_json_from_boagent_response() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .with_body_from_file("../mocks/boagent_response.json")
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        let deserialized_json_result = deserialize_boagent_json(response).await;

        assert!(deserialized_json_result
            .as_ref()
            .is_ok_and(|response| response.is_object()));
        assert!(
            deserialized_json_result.as_ref().unwrap()["raw_data"]["hardware_data"].is_object()
        );
        assert!(
            deserialized_json_result.as_ref().unwrap()["raw_data"]["boaviztapi_data"].is_object()
        );
        assert!(deserialized_json_result.as_ref().unwrap()["raw_data"]["power_data"].is_object());
    }

    #[sqlx::test]
    async fn it_gets_all_process_ids_for_processes_available_from_boagent_response() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .with_body_from_file("../mocks/boagent_response.json")
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        let deserialized_json_result = deserialize_boagent_json(response).await.unwrap();

        let processes_ids = get_processes_ids(deserialized_json_result);

        assert!(processes_ids.is_ok());
        assert_eq!(processes_ids.unwrap().len(), 10);
    }
}
