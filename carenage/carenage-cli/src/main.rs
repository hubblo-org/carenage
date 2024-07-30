use database::timestamp;
use clap::Parser;
use std::process::Command;

pub mod cli;

fn main() {
    let cli = cli::Cli::parse();

    /* let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data");
    let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts");
    let lifetime_env = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.");
    let lifetime = lifetime_env
        .parse::<i16>()
        .expect("Unable to parse LIFETIME environment variable. It must be an integer.");
    let device_name = var("DEVICE").unwrap_or("unknown".to_string());
    let database_url = var("DATABASE_URL").expect("DATABASE_URL environment variable is absent.");

    let printable_boagent_url = boagent_url.clone(); */

    let start_timestamp: timestamp::Timestamp = timestamp::Timestamp::new(cli.unix);
    let stop_timestamp: timestamp::Timestamp;

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            stop_timestamp = timestamp::Timestamp::new(cli.unix);
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("Start event timestamp is {:?}", start_timestamp.to_string());
            Command::new("./target/debug/carenaged").spawn().expect("failed to fork carenaged"); 
        }
        Some(cli::Events::Stop) => {
            let stop_timestamp = timestamp::Timestamp::new(cli.unix);

            let printable_start_timestamp = start_timestamp.to_string();
            let printable_stop_timestamp = stop_timestamp.to_string();
            println!("Carenage stop event");
            println!("Stop event timestamp is {:?}", printable_stop_timestamp);
        }
        None => {
            println!("Unknown command")
        }
    }
}
