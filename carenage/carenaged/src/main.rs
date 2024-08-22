use database::{
    connect_to_database, deserialize_boagent_json, format_hardware_data, insert_device_metadata,
};
use database::{query_boagent, timestamp::Timestamp};
use dotenv::var;
use std::{env, process};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{self, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sigterm = signal(SignalKind::terminate())?;
    let args: Vec<String> = env::args().collect();
    let time_step: u64 = args[1].parse().expect("Failed to parse time_step.");
    let start_time_str = args[2].to_string();
    let is_unix_set: bool = args[3]
        .parse()
        .expect("Failed to parse is_unix_set variable.");

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

    println!("Started carenage daemon with PID: {}", process::id());

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

async fn query_and_insert_data(
    start_time: Timestamp,
    is_unix_set: bool,
    fetch_hardware: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let boagent_url = var("BOAGENT_URL")?;    
    let location = var("LOCATION")?;
    let lifetime: i16 = var("LIFETIME")?.parse().expect("Failed to parse lifetime value.");
    let device_name = var("DEVICE").unwrap_or("unknown".to_string());
    let database_url = var("DATABASE_URL")?;
    let end_time = Timestamp::new(is_unix_set);

    let response = query_boagent(
        boagent_url,
        start_time,
        end_time,
        fetch_hardware,
        location.clone(),
        lifetime,
    )
    .await?;
    let deserialized_response = deserialize_boagent_json(response).await?;

    match fetch_hardware {
        true => {
            let device_data =
                format_hardware_data(deserialized_response, device_name, location, lifetime)?;
            let database_connection = connect_to_database(database_url).await?;
            let insert_device_data = insert_device_metadata(database_connection, device_data).await;
            match insert_device_data {
                Ok(()) => (),
                Err(err) => {
                    eprintln!(
                        "Error while processing first query to project and device tables: {}",
                        err
                    );
                    process::exit(0x0100)
                }
            }
        }
        false => ()
    };
    Ok(())
}
