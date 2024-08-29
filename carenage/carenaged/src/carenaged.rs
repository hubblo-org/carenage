use database::timestamp::UnixFlag;
use database::{
    deserialize_boagent_json, format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata, query_boagent, timestamp::Timestamp, HardwareData,
};
use serde_json::json;
use std::env;
use std::process;

pub struct DaemonArgs {
    pub time_step: u64,
    pub start_timestamp: Timestamp,
    pub unix_flag: UnixFlag,
    pub init_flag: bool,
}

impl DaemonArgs {
    pub fn parse_args() -> Result<DaemonArgs, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        let time_step: u64 = args[1].parse()?;
        let start_time_str = args[2].to_string();
        let is_unix_set: bool = args[3].parse()?;
        let init_flag: bool = args[4].parse()?;
        let unix_flag = UnixFlag::from_bool(is_unix_set);
        let start_timestamp = Timestamp::parse_str(start_time_str, unix_flag);

        Ok(DaemonArgs {
            time_step,
            start_timestamp,
            unix_flag,
            init_flag,
        })
    }
}

pub async fn insert_project_metadata(
    start_timestamp: Timestamp,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = database::Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(config.database_url).await?;
    let project_data = json!({
        "name": config.project_name,
        "start_date": start_timestamp.to_string(),
    });

    let insert_project_data =
        insert_dimension_table_metadata(db_pool.acquire().await?, "projects", project_data).await;
    match insert_project_data {
        Ok(insert_project_data) => println!(
            "Inserted project metadata into database, affected rows: {}",
            insert_project_data.rows_affected()
        ),
        Err(err) => {
            eprintln!(
                "Error while processing first query to project table: {}",
                err
            );
            process::exit(0x0100)
        }
    };
    Ok(())
}

pub async fn query_and_insert_data(
    start_time: Timestamp,
    unix_flag: UnixFlag,
    fetch_hardware: HardwareData,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = database::Config::check_configuration(&project_root_path)?;

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
