use database::boagent::{deserialize_boagent_json, query_boagent, Config, HardwareData};
use database::ci::GitlabVariables;
use database::database::{
    format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata,
};
use database::timestamp::{Timestamp, UnixFlag};
use database::tables::{Insert, Create, CarenageRow};
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

    let insert_project_metadata_query = insert_dimension_table_metadata(
        db_pool.acquire().await?,
        CarenageRow::Project.table_name(),
        CarenageRow::Project.serialize(start_timestamp),
    )
    .await;
    let project_id = get_project_id(
        insert_project_metadata_query,
        gitlab_vars.project_path.clone(),
    )
    .await?;

    let workflow_rows = CarenageRow::Workflow.insert(start_timestamp).await?;
    let workflow_id = CarenageRow::Workflow.get_id(workflow_rows);

    let pipeline_rows = CarenageRow::Pipeline.insert(start_timestamp).await?;
    let pipeline_id = CarenageRow::Pipeline.get_id(pipeline_rows);

    let job_rows = CarenageRow::Job.insert(start_timestamp).await?;
    let job_id = CarenageRow::Job.get_id(job_rows);

    let run_rows = CarenageRow::Run.insert(start_timestamp).await?;
    let run_id = CarenageRow::Run.get_id(run_rows);

    let task_rows = CarenageRow::Task.insert(start_timestamp).await?;
    let task_id = CarenageRow::Task.get_id(task_rows);

    let id_vector: Vec<uuid::Uuid> = vec![
        project_id,
        workflow_id,
        pipeline_id,
        job_id,
        run_id,
        task_id,
    ];
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
            project_rows[0].get("id")
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
                select_project_id_query.get("id")
            }
            _ => {
                eprintln!("Error while processing metadata insertion: {}", err);
                process::exit(0x0100)
            }
        },
    };

    Ok(project_id)
}
