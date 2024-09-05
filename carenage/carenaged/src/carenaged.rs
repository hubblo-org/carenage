use database::boagent::{deserialize_boagent_json, query_boagent, Config, HardwareData};
use database::ci::GitlabVariables;
use database::database::{
    format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata,
};
use database::timestamp::{Timestamp, UnixFlag};
use serde_json::json;
use sqlx::error::ErrorKind;
use sqlx::postgres::PgRow;
use sqlx::types::uuid;
use sqlx::Row;
use std::env;
use std::process;

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

pub async fn insert_project_metadata(
    gitlab_vars: GitlabVariables,
    start_timestamp: Timestamp,
) -> Result<Vec<uuid::Uuid>, Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(config.database_url).await?;
    let project_data = json!({
        "name": gitlab_vars.project_path,
        "start_date": start_timestamp.to_string(),
    });

    let insert_project_metadata_query =
        insert_dimension_table_metadata(db_pool.acquire().await?, "projects", project_data).await;
    let project_id = get_project_id(
        insert_project_metadata_query,
        gitlab_vars.project_path.clone(),
    )
    .await?;

    let workflow_name = format!("workflow_{}", gitlab_vars.project_path);
    let workflow_data = json!({
    "name": workflow_name,
    "start_date": gitlab_vars.pipeline_created_at.to_string(),
    });
    let workflow_rows =
        insert_dimension_table_metadata(db_pool.acquire().await?, "workflows", workflow_data)
            .await?;
    let workflow_id = workflow_rows[0].get("workflow_id");

    let pipeline_data = json!({
    "name": gitlab_vars.pipeline_name,
    "start_date": start_timestamp.to_string(),
    });
    let pipeline_rows =
        insert_dimension_table_metadata(db_pool.acquire().await?, "pipelines", pipeline_data)
            .await?;
    let pipeline_id: uuid::Uuid = pipeline_rows[0].get("pipeline_id");

    let job_data = json!({
    "name": gitlab_vars.job_name.to_string(),
    "start_date": gitlab_vars.job_started_at.to_string(),
    });
    let job_rows =
        insert_dimension_table_metadata(db_pool.acquire().await?, "jobs", job_data).await?;
    let job_id: uuid::Uuid = job_rows[0].get("job_id");

    let run_name = format!("run_{}", gitlab_vars.job_name);
    let run_data = json!({
    "name": run_name,
    "start_date": start_timestamp.to_string()
    });
    let run_rows =
        insert_dimension_table_metadata(db_pool.acquire().await?, "runs", run_data).await?;
    let run_id: uuid::Uuid = run_rows[0].get("run_id");

    let task_data = json!({
    "name": gitlab_vars.job_stage.to_string(),
    "start_date": start_timestamp.to_string()
    });
    let task_rows = 
        insert_dimension_table_metadata(db_pool.acquire().await?, "tasks", task_data).await?;
    let task_id: uuid::Uuid = task_rows[0].get("task_id");

    let id_vector: Vec<uuid::Uuid> = vec![project_id, workflow_id, pipeline_id, job_id, run_id, task_id];
    Ok(id_vector)
}

// Will return device_id on first_query, implement option for fn result
pub async fn query_and_insert_data(
    metadata_ids: Option<&Vec<uuid::Uuid>>,
    start_time: Timestamp,
    unix_flag: UnixFlag,
    fetch_hardware: HardwareData,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let deserialized_response = deserialize_boagent_json(response).await?;
    let db_pool = get_db_connection_pool(config.database_url).await?;

    if let HardwareData::Inspect = fetch_hardware {
        let device_data = format_hardware_data(
            deserialized_response,
            config.device_name,
            config.location,
            config.lifetime,
        )?;
        let insert_device_data =
            insert_device_metadata(db_pool.acquire().await?, device_data).await;
        match insert_device_data {
            Ok(_insert_device_data) => println!("Inserted device data into database."),
            Err(err) => {
                eprintln!(
                    "Error while processing first query to device table: {}",
                    err
                );
                process::exit(0x0100)
            }
        }
    };
    Ok(())
}

async fn get_project_id(
    insert_attempt: Result<Vec<PgRow>, sqlx::Error>,
    project_name: String,
) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(config.database_url).await?;

    let project_id: uuid::Uuid = match insert_attempt {
        Ok(project_rows) => {
            println!("Inserted project metadata into database.",);
            project_rows[0].get("project_id")
        }
        Err(err) => match err
            .as_database_error()
            .expect("It should be a DatabaseError")
            .kind()
        {
            ErrorKind::UniqueViolation => {
                println!(
                    "Metadata already present in database, not a project initialization: {}",
                    err
                );
                let select_project_id_query =
                    database::database::get_project_id(db_pool.acquire().await?, project_name)
                        .await?;
                select_project_id_query.get("project_id")
            }
            _ => {
                eprintln!("Error while processing metadata insertion: {}", err);
                process::exit(0x0100)
            }
        },
    };

    Ok(project_id)
}
