use std::env;
use crate::timestamp::{Timestamp, UnixFlag};

pub struct GitlabVariables {
    pub project_path: String,
    pub pipeline_id: u64,
    pub pipeline_created_at: Timestamp,
    pub pipeline_name: String,
    pub job_name: String,
    pub job_stage: String,
    pub job_started_at: Timestamp,
}

impl GitlabVariables {
    pub fn parse_env_variables() -> Result<GitlabVariables, Box<dyn std::error::Error>> {
        let project_path = env::var("CI_PROJECT_PATH")?.to_string();
        let pipeline_id = env::var("CI_PIPELINE_ID")?.to_string().parse::<u64>()?;
        let pipeline_created_at = Timestamp::parse_str(env::var("CI_PIPELINE_CREATED_AT")?.to_string(), UnixFlag::Unset);
        let pipeline_name = env::var("CI_PIPELINE_NAME")?.to_string();
        let job_name = env::var("CI_JOB_NAME")?.to_string();
        let job_stage = env::var("CI_JOB_STAGE")?.to_string();
        let job_started_at = Timestamp::parse_str(env::var("CI_JOB_STARTED_AT")?.to_string(), UnixFlag::Unset);

        Ok(GitlabVariables {
            project_path,
            pipeline_id,
            pipeline_created_at,
            pipeline_name,
            job_name,
            job_stage,
            job_started_at,
        })
    }
}
