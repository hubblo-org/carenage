use crate::carenaged::{insert_metadata, insert_event, query_and_insert_event};
use carenaged::DaemonArgs;
use database::boagent::HardwareData;
use database::ci::GitlabVariables;
use database::event::{EventBuilder, EventType};
use log::{error, info};
use std::process;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{self, Duration};

pub mod carenaged;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Started carenage daemon with PID: {}", process::id());

    let mut sigterm = signal(SignalKind::terminate())?;
    let args = DaemonArgs::parse_args()?;

    info!("Time step is : {} seconds.", args.time_step);
    info!("Start timestamp is {}.", args.start_timestamp);
    info!("{}", args.unix_flag);

    let gitlab_vars = GitlabVariables::parse_env_variables().expect("Gitlab variables are not available.");

    let project_ids = insert_metadata(gitlab_vars, args.start_timestamp, args.unix_flag).await?;

    let start_event = EventBuilder::new(project_ids, EventType::Start).build();
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
            info!("Received SIGTERM signal.");
            info!("Stopped carenage daemon.");
            process::exit(0x0100);
        }
        _ => {
            error!("Unable to listen to SIGTERM signal.")
        }
    }
    Ok(())
}
