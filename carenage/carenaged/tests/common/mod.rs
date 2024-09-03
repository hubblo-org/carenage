use std::env;
use chrono::Local;

pub fn setup() {
    let now = Local::now();
    env::set_var("CI_PROJECT_PATH", "hubblo/carenage");
    env::set_var("CI_PIPELINE_ID", "1234");
    env::set_var("CI_PIPELINE_CREATED_AT", now.to_string());
    env::set_var("CI_PIPELINE_NAME", "Pipeline for merge request");
    env::set_var("CI_JOB_NAME", "build_env_and_test");
    env::set_var("CI_JOB_STAGE", "test");
    env::set_var("CI_JOB_STARTED_AT", now.to_string());
}
