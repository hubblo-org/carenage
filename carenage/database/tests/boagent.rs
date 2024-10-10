use chrono::{Duration, Local};
use database::boagent::{
    deserialize_boagent_json, get_processes_ids, process_embedded_impacts, query_boagent,
    HardwareData,
};
use database::timestamp::Timestamp;
use mockito::{Matcher, Server};
use std::time::SystemTime;

#[sqlx::test]
async fn it_queries_boagent_with_success_with_needed_query_parameters() {
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
        Timestamp::Unix(Some(now_timestamp_minus_one_minute)),
        Timestamp::Unix(Some(now_timestamp)),
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
        Timestamp::Unix(None),
        Timestamp::Unix(None),
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
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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
        Timestamp::ISO8601(None),
        Timestamp::ISO8601(None),
        HardwareData::Inspect,
        "FRA".to_string(),
        5,
    )
    .await;

    assert!(response.is_err());
}

#[sqlx::test]
async fn it_deserializes_json_from_boagent_response() {
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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
    assert!(deserialized_json_result.as_ref().unwrap()["raw_data"]["hardware_data"].is_object());
    assert!(deserialized_json_result.as_ref().unwrap()["raw_data"]["boaviztapi_data"].is_object());
    assert!(deserialized_json_result.as_ref().unwrap()["raw_data"]["power_data"].is_object());
}

#[sqlx::test]
async fn it_gets_all_process_ids_for_processes_available_from_boagent_response() {
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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

#[sqlx::test]
async fn it_queries_process_embedded_impacts_from_boagent_with_returned_ids() {
    let pids = [6042, 4163, 171690, 4489, 7281, 7868, 5567, 5365, 810, 14063];
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

    let mut boagent_server = Server::new_async().await;

    let url = boagent_server.url();

    for pid in pids {
        let _mock = boagent_server
            .mock("GET", "/process_embedded_impacts")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("process_id".to_string(), pid.to_string()),
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
            .with_body_from_file("../mocks/process_embedded_impacts.json")
            .create_async()
            .await;

        let response = process_embedded_impacts(
            url.clone(),
            pid,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await;

        assert!(response.is_ok())
    }
}
