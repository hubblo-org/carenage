use database::boagent::{
    deserialize_boagent_json, process_embedded_impacts, query_boagent, Config, HardwareData,
};
use database::ci::GitlabVariables;
use database::database::{collect_processes, get_db_connection_pool, insert_metrics, Ids};
use database::event::{Event, EventType};
use database::metrics::Metrics;
use database::tables::Process;
use database::tables::{CarenageRow, Metadata};
use database::timestamp::{Timestamp, UnixFlag};
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

    let start_process = Process {
        pid: process::id() as i32,
        exe: "carenage".to_string(),
        cmdline: "carenage start".to_string(),
        state: "running".to_string(),
        start_date: start_timestamp.to_string(),
    };

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
    Ok(println!("Inserted event data into database."))
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

    let processes = collect_processes(&deserialized_boagent_response, start_time)
        .expect("Processes data should be collected from Scaphandre.");

    for process in processes {
        let db_pool = get_db_connection_pool(&config.database_url).await?;
        let process_row = Process::insert(&process, db_pool.acquire().await?).await?;
        let process_id = Process::get_id(process_row);
        ids.process_id = process_id;

        let event = Event::build(ids, EventType::Regular);
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

        let metrics = Metrics::build(&process_data, &deserialized_boagent_response)?;

        insert_metrics(event_id, db_pool.acquire().await?, metrics).await?;
    }

    Ok(println!("Inserted all metrics for query."))
}
