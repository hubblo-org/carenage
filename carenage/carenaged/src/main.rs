use crate::carenaged::{insert_project_metadata, query_and_insert_data};
use carenaged::DaemonArgs;
use std::process;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{self, Duration};

pub mod carenaged;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Started carenage daemon with PID: {}", process::id());

    let mut sigterm = signal(SignalKind::terminate())?;
    let args = DaemonArgs::parse_args()?;

    println!("Time step is : {} seconds.", args.time_step);
    println!("Start timestamp is {}.", args.start_timestamp);
    println!("{}", args.unix_flag);

    if args.init_flag {
            let _ = insert_project_metadata(args.start_timestamp).await;
            print!("Project initialization, inserted project metadata into Carenage database.")
    }

    let _first_query = query_and_insert_data(args.start_timestamp, args.unix_flag, true).await;

    let _query_insert_loop = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(args.time_step));
        loop {
            let _ = query_and_insert_data(args.start_timestamp, args.unix_flag, false).await;
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

async fn insert_project_metadata(start_timestamp: Timestamp) -> Result<(), Box<dyn std::error::Error>> {
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

async fn query_and_insert_data(
    start_time: Timestamp,
    unix_flag: UnixFlag,
    fetch_hardware: bool,
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

    if fetch_hardware {
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
