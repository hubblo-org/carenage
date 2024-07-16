use clap::Parser;
use crate::database::Timestamp;
use dotenv::var;
use crate::database::query_boagent;

pub mod cli;
pub mod database;

fn main() {
    let cli = cli::Cli::parse();

    let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent");
    let printable_boagent_url = boagent_url.clone();

    let start_timestamp: Timestamp;
    let stop_timestamp: Timestamp;

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            start_timestamp = cli::create_timestamp(cli.unix);
            stop_timestamp = cli::create_timestamp(cli.unix);
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("Start event timestamp is {:?}", start_timestamp.to_string());
            let boagent_query = query_boagent(
                boagent_url,
                start_timestamp,
                stop_timestamp,
                true,
                "FRA".to_string(),
                5,
            );
                match boagent_query {
                    Ok(response) => println!("Queried Boagent at {:?} : {:?}", printable_boagent_url, response),
                    Err(err) => println!("Failed to query Boagent: {:?}", err),
                };
        }
        Some(cli::Events::Stop) => {
            let timestamp = cli::create_timestamp(cli.unix);
            println!("Carenage stop event");
            println!("Stop event timestamp is {:?}", timestamp.to_string())
        }
        None => {
            println!("Unknown command")
        }
    }
}
