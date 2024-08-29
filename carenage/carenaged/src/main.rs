use crate::carenaged::{insert_project_metadata, query_and_insert_data};
use carenaged::DaemonArgs;
use database::HardwareData;
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

    let _first_query = query_and_insert_data(args.start_timestamp, args.unix_flag, HardwareData::Inspect).await;

    let _query_insert_loop = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(args.time_step));
        loop {
            let _ = query_and_insert_data(args.start_timestamp, args.unix_flag, HardwareData::Ignore).await;
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
