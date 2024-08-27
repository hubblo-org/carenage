use chrono::Utc;
use database::{
    deserialize_boagent_json, format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata, query_boagent, timestamp::Timestamp,
};
use serde_json::json;
use std::{env, process};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{self, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Started carenage daemon with PID: {}", process::id());

    let mut sigterm = signal(SignalKind::terminate())?;
    let args: Vec<String> = env::args().collect();
    let time_step: u64 = args[1]
        .parse()
        .expect("time_step variable should be parsable.");
    let start_time_str = args[2].to_string();
    let is_unix_set: bool = args[3]
        .parse()
        .expect("is_unix_set variable should be parsable.");
    let is_init_set: bool = args[4]
        .parse()
        .expect("is_init_set variable should be parsable.");

    println!("Time step is : {} seconds.", time_step);
    println!("Start timestamp is {}.", start_time_str);
    println!("Is UNIX flag set for timestamp? {}", is_unix_set);

    let start_time_timestamp: Timestamp = match is_unix_set {
        true => Timestamp::UnixTimestamp(Some(
            start_time_str
                .parse::<u64>()
                .expect("Failed to parse string to convert to UNIX epoch timestamp."),
        )),
        false => Timestamp::ISO8601Timestamp(Some(
            start_time_str
                .parse()
                .expect("The string should be parsable to convert it to ISO8601 timestamp."),
        )),
    };

    match is_init_set {
        true => {
            let _ = insert_project_metadata().await;
            print!("Project initialization, inserted project metadata into Carenage database.")
        }
        false => (),
    }


    // TODO : first query to get hardware_data, format it, send to database with other project
    // metadata
    // Exit process if database error
    let _first_query = query_and_insert_data(start_time_timestamp, is_unix_set, true).await;

    // Loop to query and insert data for events table
    let _query = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(time_step));
        loop {
            let _ = query_and_insert_data(start_time_timestamp, is_unix_set, false).await;
            interval.tick().await;
        }
    });

    match sigterm.recv().await {
        Some(()) => {
            println!("Received SIGTERM signal.");
            println!("Stopped carenage daemon.");
            process::exit(0x0100);
        }
        _ => {
            eprintln!("Unable to listen to SIGTERM signal.")
        }
    }
    Ok(())
}

async fn insert_project_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = database::Config::check_configuration(&project_root_path)?;
    let db_pool = get_db_connection_pool(config.database_url).await?;
    let start_date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();    
    let project_data = json!({
        "name": config.project_name,
        "start_date": start_date,
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

async fn query_and_insert_data(
    start_time: Timestamp,
    is_unix_set: bool,
    fetch_hardware: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = database::Config::check_configuration(&project_root_path)?;

    let end_time = Timestamp::new(is_unix_set);
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

    match fetch_hardware {
        true => {
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
        }
        false => (),
    };
    Ok(())
}
