use clap::{Parser, Subcommand};

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
    /// Project initialization: this flag allows to record project metadata
    #[arg(short, long, default_value_t = false)]
    pub init: bool,
}

#[derive(Subcommand)]
pub enum Events {
    /// Start carenage, with an optional time step
    Start(StartArgs),

    /// Stop carenage, final event
    Stop,
}
