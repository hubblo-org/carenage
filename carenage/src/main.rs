use chrono::Utc;
use clap::Parser;
use std::time::SystemTime;
use crate::database::{Timestamp, query_boagent};

pub mod cli;
pub mod database;

fn main() -> Result<(), ()> {
    let cli = cli::Cli::parse();

    let start_timestamp: Timestamp;
    let stop_timestamp: Timestamp;
    let boagent_url = "http://127.0.0.1:3000".to_string();

    match cli.unix {
        true => {
            start_timestamp = Timestamp::UnixTimestamp(Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ));
            stop_timestamp = Timestamp::UnixTimestamp(Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ));
        }
        false => {
            start_timestamp = Timestamp::ISO8601Timestamp(Utc::now());
            stop_timestamp = Timestamp::ISO8601Timestamp(Utc::now());
        }
    }

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("{:?}", start_timestamp.to_string())
        }
        Some(cli::Events::Stop) => {
            println!("Carenage stop event");
            println!("{:?}", stop_timestamp.to_string())
        }
        None => {
            println!("Unknown command")
        }
    }

    let boagent_query = query_boagent(
        boagent_url,
        start_timestamp,
        stop_timestamp,
        true,
        "FRA".to_string(),
        5,
    );

    match boagent_query {
        Ok(response) => Ok(println!("Queried Boagent: {:?}", response)),
        Err(err) => {
            println!("Failed to query Boagent: {:?}", err);
            Ok(())
        }
    }
}
