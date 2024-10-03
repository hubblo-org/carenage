use database::boagent::{deserialize_boagent_json, query_boagent, Config, HardwareData};
use database::ci::GitlabVariables;
use database::tables::{CarenageRow, Insert};
use database::timestamp::{Timestamp, UnixFlag};
use sqlx::types::{uuid, Uuid};
use std::env;

pub struct DaemonArgs {
    pub time_step: u64,
    pub start_timestamp: Timestamp,
    pub unix_flag: UnixFlag,
}

impl DaemonArgs {
    pub fn parse_args() -> Result<DaemonArgs, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        let time_step: u64 = args[1].parse()?;
        let start_time_str = args[2].to_string();
        let is_unix_set: bool = args[3].parse()?;
        let unix_flag: UnixFlag = is_unix_set.into();
        let start_timestamp = Timestamp::parse_str(start_time_str, unix_flag);

        Ok(DaemonArgs {
            time_step,
            start_timestamp,
            unix_flag,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Ids {
    pub project_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub device_id: Uuid,
}

pub async fn insert_metadata(
    gitlab_vars: GitlabVariables,
    start_timestamp: Timestamp,
    unix_flag: UnixFlag,
) -> Result<Ids, Box<dyn std::error::Error>> {
    let project_rows = CarenageRow::Project.insert(start_timestamp, None).await?;
    let project_id = CarenageRow::Project
        .get_id(project_rows, Some(gitlab_vars.project_path.clone()))
        .await?;

    let workflow_rows = CarenageRow::Workflow.insert(start_timestamp, None).await?;
    let workflow_id = CarenageRow::Workflow.get_id(workflow_rows, None).await?;

    let pipeline_rows = CarenageRow::Pipeline.insert(start_timestamp, None).await?;
    let pipeline_id = CarenageRow::Pipeline.get_id(pipeline_rows, None).await?;

    let job_rows = CarenageRow::Job.insert(start_timestamp, None).await?;
    let job_id = CarenageRow::Job.get_id(job_rows, None).await?;

    let run_rows = CarenageRow::Run.insert(start_timestamp, None).await?;
    let run_id = CarenageRow::Run.get_id(run_rows, None).await?;

    let task_rows = CarenageRow::Task.insert(start_timestamp, None).await?;
    let task_id = CarenageRow::Task.get_id(task_rows, None).await?;

    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;

    let end_time = Timestamp::new(unix_flag);
    let response = query_boagent(
        config.boagent_url,
        start_timestamp,
        end_time,
        HardwareData::Inspect,
        config.location.clone(),
        config.lifetime,
    )
    .await?;
    let deserialized_boagent_response = deserialize_boagent_json(response).await?;
    let insert_device_data = CarenageRow::Device
        .insert(start_timestamp, Some(deserialized_boagent_response))
        .await?;
    let device_id = CarenageRow::Device.get_id(insert_device_data, None).await?;

    let ids = Ids {
        project_id,
        workflow_id,
        pipeline_id,
        job_id,
        run_id,
        task_id,
        device_id,
    };
    Ok(ids)
}

// Will return device_id on first_query, implement option for fn result

pub async fn query_and_insert_data(
    ids: Ids,
    start_time: Timestamp,
    unix_flag: UnixFlag,
    fetch_hardware: HardwareData,
) -> Result<Ids, Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;

    let end_time = Timestamp::new(unix_flag);
    let response = query_boagent(
        config.boagent_url,
        start_time,
        end_time,
        fetch_hardware,
        config.location.clone(),
        config.lifetime,
    )
    .await?;
    let deserialized_boagent_response = deserialize_boagent_json(response).await?;
    todo!()
}
