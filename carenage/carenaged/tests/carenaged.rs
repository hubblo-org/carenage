use carenaged::carenaged::{insert_metadata, query_and_insert_data};
use chrono::Local;
use database::boagent::HardwareData;
use database::ci::GitlabVariables;
use database::timestamp::{Timestamp, UnixFlag};
use mockito::{Matcher, Server};
use std::env;
mod common;

#[tokio::test]
async fn it_inserts_project_metadata_when_needed_gitlab_variables_are_available() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    env::set_var("BOAGENT_URL", url);

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
        .with_body_from_file("../mocks/boagent_response.json")
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
    env::set_var("BOAGENT_URL", url);
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
        .with_body_from_file("../mocks/boagent_response.json")
        .create_async()
        .await;

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let insert_result = insert_metadata(gitlab_vars, now, UnixFlag::Unset).await;

    assert!(insert_result.is_ok())
}

#[tokio::test]
async fn it_inserts_needed_foreign_keys_and_data_to_events_table() {
    common::setup();
    let now = Timestamp::new(UnixFlag::Unset);
    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();

    let mut boagent_server = Server::new_async().await;
    let url = boagent_server.url();
    env::set_var("BOAGENT_URL", url);
    let mock_boagent_response = boagent_server
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
        .with_body_from_file("../mocks/boagent_response.json")
        .create_async()
        .await;
    let insert_result = insert_metadata(gitlab_vars, now, UnixFlag::Unset).await;
    let ids = insert_result.unwrap();

    let query = query_and_insert_data(ids, now, UnixFlag::Unset, HardwareData::Ignore).await;
    assert!(query.is_ok());
    todo!()
}
