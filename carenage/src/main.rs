use crate::database::query_boagent;
use crate::database::Timestamp;
use clap::Parser;
use dotenv::var;

pub mod cli;
pub mod database;

fn main() {
    let cli = cli::Cli::parse();

    let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data");
    let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts");
    let lifetime_env = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.");
    let lifetime = lifetime_env.parse::<u8>().expect("Unable to parse LIFETIME environment variable. It must be an integer.");

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
                Ok(response) => println!(
                    "Queried Boagent at {:?} : {:?}",
                    printable_boagent_url, response
                ),
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
