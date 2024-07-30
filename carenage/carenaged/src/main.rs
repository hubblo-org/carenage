use std::process;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sigterm = signal(SignalKind::terminate())?;

    loop {
        println!("Started carenage daemon with PID: {}", process::id());
        sigterm.recv().await;
        println!("Received SIGTERM signal");
        println!("Stopped carenage daemon.");
        break;
    }
    Ok(())
}
