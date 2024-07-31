use clap::Parser;
use database::timestamp;
use std::{fs::File, io::Write, process::Command};
use sysinfo::{Pid, Signal, System};

pub mod cli;

fn main() {
    let cli = cli::Cli::parse();

    /*     let device_name = var("DEVICE").unwrap_or("unknown".to_string());
    let database_url = var("DATABASE_URL").expect("DATABASE_URL environment variable is absent.");

    let printable_boagent_url = boagent_url.clone(); */

    match &cli.event {
        Some(cli::Events::Start(step)) => {
            let start_timestamp: timestamp::Timestamp = timestamp::Timestamp::new(cli.unix);
            println!("Carenage start event, time step of {:?} seconds", step.step);
            println!("Start event timestamp is {:?}", start_timestamp.to_string());
            let carenaged = Command::new("./target/debug/carenaged")
                .arg(step.step.to_string())
                .arg(start_timestamp.to_string())
                .arg(cli.unix.to_string())
                .spawn()
                .expect("Failed to fork carenaged.");
            let carenaged_pid = carenaged.id().to_string();
            let mut pid_file =
                File::create("pid.txt").expect("Failed to create file to save child process PID.");
            pid_file
                .write_all(carenaged_pid.as_bytes())
                .expect("Failed to save child process PID in file.");
        }
        Some(cli::Events::Stop) => {
            let stop_timestamp = timestamp::Timestamp::new(cli.unix);
            let system = System::new_all();

            let printable_stop_timestamp = stop_timestamp.to_string();
            println!("Carenage stop event");
            println!("Stop event timestamp is {:?}", printable_stop_timestamp);
            let pid_file = std::fs::read_to_string("pid.txt")
                .expect("Failed to open file with saved child process PID.");
            let pid = pid_file
                .parse::<u32>()
                .expect("Failed to convert pid.txt contents to u32.");
            let carenaged_process = system
                .process(Pid::from_u32(pid))
                .expect("Failed to get process with given PID");
            carenaged_process
                .kill_with(Signal::Term)
                .expect("Failed to kill with SIGTERM");
        }
        None => {
            println!("Unknown command")
        }
    }
}
