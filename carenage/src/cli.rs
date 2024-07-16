use crate::database::Timestamp;
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::time::SystemTime;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub event: Option<Events>,

    /// Format timestamp to Unix epoch, in seconds
    #[arg(short, long)]
    pub unix: bool,
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    /// Time step in seconds between events
    #[arg(short, long, default_value_t = 5)]
    pub step: u64,
}

#[derive(Subcommand)]
pub enum Events {
    /// Start carenage, with an optional time step
    Start(StartArgs),

    /// Stop carenage, final event
    Stop,
}

pub fn create_timestamp(unix: bool) -> Timestamp {
    match unix {
        true => Timestamp::UnixTimestamp(Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )),
        false => Timestamp::ISO8601Timestamp(Some(Utc::now())),
    }
}
