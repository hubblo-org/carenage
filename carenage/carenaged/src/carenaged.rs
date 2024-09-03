use database::boagent::{deserialize_boagent_json, query_boagent, Config, HardwareData};
use database::ci::GitlabVariables;
use database::database::{
    format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata,
};
use database::timestamp::{Timestamp, UnixFlag};
use serde_json::json;
use sqlx::error::ErrorKind;
use sqlx::postgres::PgQueryResult;
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
        let unix_flag = UnixFlag::from_bool(is_unix_set);
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
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(config.database_url).await?;
    let project_data = json!({
        "name": gitlab_vars.project_path,
        "start_date": start_timestamp.to_string(),
    });

    let insert_project_data =
        insert_dimension_table_metadata(db_pool.acquire().await?, "projects", project_data).await;

    let _result = check_unique_constraint(insert_project_data);

    let workflow_name = format!("workflow_{}", gitlab_vars.project_path);
    let workflow_data = json!({
    "name": workflow_name,
    "start_date": gitlab_vars.pipeline_created_at.to_string(),
    });

    let insert_workflow_data =
        insert_dimension_table_metadata(db_pool.acquire().await?, "workflows", workflow_data).await;
    
    let _result = check_unique_constraint(insert_workflow_data);

    let job_data = json!({
    "name": gitlab_vars.job_name.to_string(),
    "start_date": gitlab_vars.job_started_at.to_string(),
    });

    let insert_job_data =
        insert_dimension_table_metadata(db_pool.acquire().await?, "jobs", job_data).await;

    Ok(())
}

pub async fn query_and_insert_data(
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
            Ok(()) => (),
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

fn check_unique_constraint(
    insert_attempt: Result<PgQueryResult, sqlx::Error>,
) -> Result<(), Box<dyn std::error::Error>> {
    match insert_attempt {
        Ok(insert_attempt) => println!(
            "Inserted metadata into database, affected rows: {}",
            insert_attempt.rows_affected()
        ),
        Err(err) => match err
            .as_database_error()
            .expect("It should be a DatabaseError")
            .kind()
        {
            ErrorKind::UniqueViolation => {
                println!("Metadata already present in database, not a project initialization: {}", err)
            }
            _ => {
                eprintln!("Error while processing metadata insertion: {}", err);
                process::exit(0x0100)
            }
        },
    }

    Ok(())
}
