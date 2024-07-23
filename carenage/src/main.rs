use crate::database::query_boagent;
use crate::timestamp::Timestamp;
use clap::Parser;
use dotenv::var;
use std::path::Path;
use std::fs::File;

pub mod cli;
pub mod database;
pub mod timestamp;

fn main() {
    let cli = cli::Cli::parse();

    let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data");
    let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts");
    let lifetime_env = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.");
    let lifetime = lifetime_env
        .parse::<u8>()
        .expect("Unable to parse LIFETIME environment variable. It must be an integer.");
    let boagent_response_fp = var("BOAGENT_RESPONSE_FP").expect("BOAGENT_RESPONSE_FP environment variable is absent. It is needed to parse device data");

    let printable_boagent_url = boagent_url.clone();

    let start_timestamp: Timestamp = Timestamp::new(cli.unix);
    let stop_timestamp: Timestamp;

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            stop_timestamp = Timestamp::new(cli.unix);
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("Start event timestamp is {:?}", start_timestamp.to_string());
            let boagent_query = query_boagent(
                boagent_url,
                start_timestamp,
                stop_timestamp,
                true,
                location,
                lifetime,
            );
            match boagent_query {
                Ok(response) => {
                    println!(
                        "Queried Boagent at {:?} : {:?}",
                        printable_boagent_url, response
                    );
                let _save_response = database::save_boagent_response(response, Path::new(&boagent_response_fp)).expect("Failed to save Boagent response to filepath."); 
                let deserialized_response = database::deserialize_boagent_json(File::open(boagent_response_fp).expect("Failed to open file."));

                }
                Err(err) => println!("Failed to query Boagent: {:?}", err),
            };
        }
        Some(cli::Events::Stop) => {
            let stop_timestamp = Timestamp::new(cli.unix);

            let printable_start_timestamp = start_timestamp.to_string();
            let printable_stop_timestamp = stop_timestamp.to_string();
            println!("Carenage stop event");
            println!("Stop event timestamp is {:?}", printable_stop_timestamp);
            let boagent_query = query_boagent(
                boagent_url,
                start_timestamp,
                stop_timestamp,
                false,
                location,
                lifetime,
            );
            match boagent_query {
                Ok(response) => {
                    println!(
                        "Queried Boagent at {:?} : {:?}",
                        printable_boagent_url, response
                    );
                    println!(
                        "Carenage worked between {:?} and {:?} timestamps",
                        printable_start_timestamp, printable_stop_timestamp
                    )
                }
                Err(err) => println!("Failed to query Boagent: {:?}", err),
            };
        }
        None => {
            println!("Unknown command")
        }
    }
}
