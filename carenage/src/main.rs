use chrono::Utc;
use std::time::SystemTime;
use clap::Parser;
pub mod cli;
pub mod database;

fn main() {
    let cli = cli::Cli::parse();

    let mut start_timestamp = String::new();
    let mut stop_timestamp = String::new();

    match cli.unix {
        true => {
            start_timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            stop_timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
        }
        false => {
            start_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            stop_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("{start_timestamp:?}")
        }
        Some(cli::Events::Stop) => {
            println!("Carenage stop event");
            println!("{stop_timestamp:?}")
        }
        None => {
            println!("Unknown command")
        }
    }
}
