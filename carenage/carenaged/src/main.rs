use crate::carenaged::{insert_metadata, insert_event, query_and_insert_event};
use carenaged::DaemonArgs;
use database::boagent::HardwareData;
use database::ci::GitlabVariables;
use database::event::{Event, EventType};
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

    let gitlab_vars = GitlabVariables::parse_env_variables().expect("Gitlab variables are not available.");

    let project_ids = insert_metadata(gitlab_vars, args.start_timestamp, args.unix_flag).await?;

    let start_event = Event::build(project_ids, EventType::Start);
    insert_event(&start_event).await?;

    let _query_insert_loop = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(args.time_step));
        loop {
            let _ = query_and_insert_event(
                project_ids,
                args.start_timestamp,
                args.unix_flag,
                HardwareData::Ignore,
                EventType::Regular,
            )
            .await;
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
