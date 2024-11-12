use database::boagent::{
    deserialize_boagent_json, process_embedded_impacts, query_boagent, Config, HardwareData,
};
use database::ci::GitlabVariables;
use database::database::{check_process_existence_for_id, collect_processes, get_db_connection_pool, get_process_id, Ids};
use database::event::{Event, EventBuilder, EventType};
use database::metrics::Metrics;
use database::tables::{CarenageRow, Metadata};
use database::tables::{Process, ProcessBuilder};
use database::timestamp::{Timestamp, UnixFlag};
use log::{info, warn};
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

        info!("All needed daemon arguments are available!");

        Ok(DaemonArgs {
            time_step,
            start_timestamp,
            unix_flag,
        })
    }
}

pub async fn insert_metadata(
    gitlab_vars: GitlabVariables,
    start_timestamp: Timestamp,
    unix_flag: UnixFlag,
) -> Result<Ids, Box<dyn std::error::Error>> {
    let project_rows = CarenageRow::Project.insert(start_timestamp, None).await?;
    let project_id = CarenageRow::Project
        .get_id(project_rows, Some(&gitlab_vars.project_path))
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
        &config.boagent_url,
        start_timestamp,
        end_time,
        HardwareData::Inspect,
        &config.location,
        config.lifetime,
    )
    .await?;
    let deserialized_boagent_response = deserialize_boagent_json(response).await?;
    let insert_device_data = CarenageRow::Device
        .insert(start_timestamp, Some(deserialized_boagent_response))
        .await?;
    let device_id = CarenageRow::Device.get_id(insert_device_data, None).await?;

    let start_process = ProcessBuilder::new(
        process::id() as i32,
        "carenage",
        "carenage start",
        "running",
    )
    .build();

    let db_pool = get_db_connection_pool(&config.database_url)
        .await?
        .acquire()
        .await?;

    let process_row = Process::insert(&start_process, db_pool).await?;
    let process_id = Process::get_id(process_row);

    let ids = Ids {
        project_id,
        workflow_id,
        pipeline_id,
        job_id,
        run_id,
        task_id,
        device_id,
        process_id,
    };
    Ok(ids)
}

pub async fn insert_event(event: &Event) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(&config.database_url)
        .await?
        .acquire();

    Event::insert(event, db_pool.await?).await?;
    Ok(info!("Inserted event data into database."))
}

pub async fn query_and_insert_event(
    mut ids: Ids,
    start_time: Timestamp,
    unix_flag: UnixFlag,
    fetch_hardware: HardwareData,
    event_type: EventType,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)?;

    let end_time = Timestamp::new(unix_flag);
    let response = query_boagent(
        &config.boagent_url,
        start_time,
        end_time,
        fetch_hardware,
        &config.location,
        config.lifetime,
    )
    .await?;
    let deserialized_boagent_response = deserialize_boagent_json(response).await?;

    let processes_collection_attempt =
        collect_processes(&deserialized_boagent_response);

    /* Scaphandre, through Boagent, might not have data on processes available for the timestamps
     * sent by Carenage (notably if all of Boagent / Scaphandre / Carenage are launched at the same
     * time and Carenage is started right away). Handling the Result here and then the Option to
     * cover all possibles cases: there might be some data missing at the launch of Carenage ; or
     * there might be an error during the processing of the request. It could be relevant not to panic
     * here. */

    match processes_collection_attempt {
        Ok(Some(processes)) => {
            for process in processes {
                let db_pool = get_db_connection_pool(&config.database_url).await?;
                let process_metadata_already_registered =
                    check_process_existence_for_id(db_pool.acquire().await?, &process, "run", ids.run_id).await?;

                let process_id = match process_metadata_already_registered {
                    true => {
                        get_process_id(db_pool.acquire().await?, &process, "run", ids.run_id).await?
                    }
                    false => {
                        let process_row = Process::insert(&process, db_pool.acquire().await?).await?;
                        Process::get_id(process_row)
                    }
                };

                ids.process_id = process_id;

                let event = EventBuilder::new(ids, event_type).build();
                let event_row = Event::insert(&event, db_pool.acquire().await?).await?;
                let event_id = Event::get_id(event_row);

                let process_response = process_embedded_impacts(
                    &config.boagent_url,
                    process.pid,
                    start_time,
                    end_time,
                    fetch_hardware,
                    &config.location,
                    config.lifetime,
                )
                .await?;

                let process_data = deserialize_boagent_json(process_response).await?;

                Metrics::build(&process_data, &deserialized_boagent_response)
                    .insert(event_id, db_pool.acquire().await?)
                    .await?;
                info!("Inserted all metrics for query.");
            }
        }
        Ok(None) => info!("No processes data received yet from Scaphandre, carrying on!"),
        Err(_) => warn!(
            "Some error occured while recovering data from Scaphandre, some data might be missing!"
        ),
    }

    Ok(info!("Boagent query and metrics insertion attempt over."))
}
