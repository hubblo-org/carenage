use carenaged::carenaged::insert_project_metadata;
use database::ci::GitlabVariables;
use database::timestamp::{Timestamp, UnixFlag};
mod common;

#[tokio::test]
async fn it_inserts_project_metadata_when_needed_gitlab_variables_are_available() {
    common::setup();

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();
    let now = Timestamp::new(UnixFlag::Unset);

    let insert_result = insert_project_metadata(gitlab_vars, now).await;

    assert!(insert_result.is_ok())
}

#[tokio::test]
#[should_panic]
async fn it_fails_when_needed_gitlab_variables_are_not_available() {

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();
    let now = Timestamp::new(UnixFlag::Unset);

    let _insert_result = insert_project_metadata(gitlab_vars, now).await;

} 

#[tokio::test]
async fn it_returns_all_uuids_of_metadata_tables_to_be_used_by_events_table_as_primary_keys() {

    common::setup();

    let gitlab_vars = GitlabVariables::parse_env_variables().unwrap();
    let now = Timestamp::new(UnixFlag::Unset);

    let insert_result = insert_project_metadata(gitlab_vars, now).await;
    let ids = insert_result.unwrap();

    assert_eq!(ids.len(), 6);
}
