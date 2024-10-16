use carenaged::carenaged::{insert_event, insert_metadata, query_and_insert_event};
use chrono::Local;
use database::boagent::HardwareData;
use database::ci::GitlabVariables;
use database::event::{Event, EventType};
use database::timestamp::{Timestamp, UnixFlag};
use mockito::{Matcher, Server};
use std::env;
use std::fs::canonicalize;
mod common;

#[tokio::test]
async fn it_inserts_project_metadata_when_needed_gitlab_variables_are_available() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    env::set_var("BOAGENT_URL", url);
    let mock_boagent_path = canonicalize("../mocks/boagent_response.json").unwrap();

    let mock = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(mock_boagent_path)
        .create_async()
        .await;

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let insert_result = insert_metadata(gitlab_vars, now, UnixFlag::Unset).await;

    assert!(insert_result.is_ok())
}

#[tokio::test]
#[should_panic]
async fn it_fails_when_needed_gitlab_variables_are_not_available() {
    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();
    let now = Timestamp::ISO8601(Some(Local::now()));

    let _insert_result = insert_metadata(gitlab_vars, now, UnixFlag::Unset).await;
}
#[tokio::test]
async fn it_returns_all_uuids_of_metadata_tables_to_be_used_by_events_table_as_primary_keys() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    let mock_boagent_path = canonicalize("../mocks/boagent_response.json").unwrap();
    env::set_var("BOAGENT_URL", url);
    let _mock_boagent_query = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(mock_boagent_path)
        .create_async()
        .await;

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let insert_result = insert_metadata(gitlab_vars, now, UnixFlag::Unset).await;

    assert!(insert_result.is_ok())
}

#[tokio::test]
async fn it_inserts_start_event_to_events_table() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    let mock_boagent_path = canonicalize("../mocks/boagent_response.json").unwrap();
    env::set_var("BOAGENT_URL", url);
    let _mock_boagent_query = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(mock_boagent_path)
        .create_async()
        .await;
    let project_ids = insert_metadata(gitlab_vars, now, UnixFlag::Unset)
        .await
        .unwrap();
    let start_event = Event::build(project_ids, EventType::Start);
    let insert_event = insert_event(&start_event).await;
    assert!(insert_event.is_ok());
}

#[tokio::test]
async fn it_inserts_all_events_and_metrics_for_processes() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    let mock_boagent_path = canonicalize("../mocks/query_boagent_response_before_process_embedded_impacts.json").unwrap();
    env::set_var("BOAGENT_URL", url);

    let _mock_boagent_query_with_hardware = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(&mock_boagent_path)
        .create_async()
        .await;
    let _mock_boagent_query_without_hardware = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "false".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(&mock_boagent_path)
        .create_async()
        .await;

    let mock_process_impacts_path = canonicalize("../mocks/process6042.json").unwrap();

    let _mock_boagent_process_embedded_impacts = boagent_server
        .mock("GET", "/process_embedded_impacts")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("start_time".to_string(), now.to_string()),
            Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
            Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
            Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
            Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
            Matcher::UrlEncoded("fetch_hardware".to_string(), "false".to_string()),
        ]))
        .with_status(200)
        .with_body_from_file(&mock_process_impacts_path)
        .expect(10)
        .create_async()
        .await;

    let project_ids = insert_metadata(gitlab_vars, now, UnixFlag::Unset)
        .await
        .unwrap();
    let start_event = Event::build(project_ids, EventType::Start);
    let _ = insert_event(&start_event).await;

    let query_and_insert = query_and_insert_event(
        project_ids,
        now,
        UnixFlag::Unset,
        HardwareData::Ignore,
        EventType::Regular,
    )
    .await;
    assert!(query_and_insert.is_ok());
}
