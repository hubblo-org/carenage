use database::{query_boagent, timestamp::Timestamp};
use dotenv::var;
use std::{env, process};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::{self, Duration, Interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sigterm = signal(SignalKind::terminate())?;
    let args: Vec<String> = env::args().collect();
    let time_step: u64 = args[1].parse().expect("Failed to parse time_step");
    let start_time_str = args[2].to_string();
    let unix_is_set: bool = args[3]
        .parse()
        .expect("Failed to parse unix_is_set variable");
    let start_time_timestamp: Timestamp;

    println!("Time step is : {} seconds", time_step);
    println!("Start timestamp is {}", start_time_str);
    println!("Is UNIX flag set for timestamp? {}", unix_is_set);

    match unix_is_set {
        true => {
            start_time_timestamp = Timestamp::UnixTimestamp(Some(
                start_time_str
                    .parse::<u64>()
                    .expect("Failed to parse string to convert to POSIX timestamp."),
            ))
        }
        false => {
            start_time_timestamp = Timestamp::ISO8601Timestamp(Some(
                start_time_str
                    .parse()
                    .expect("The string should be parsable to convert it to ISO8601 timestamp."),
            ));
        }
    }

    loop {
        println!("Started carenage daemon with PID: {}", process::id());
        let query = tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(time_step));
            let _ = query_and_insert_data(start_time_timestamp).await;
            interval.tick().await;
        });
        println!("{:?}", query.await.unwrap());
        sigterm.recv().await;
        println!("Received SIGTERM signal");
        println!("Stopped carenage daemon.");
        process::exit(0x0100);
    }
    Ok(())
}

async fn query_and_insert_data(
    start_time: Timestamp,
) -> Result<String, Box<dyn std::error::Error>> {
    let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data");
    let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts");
    let lifetime: i16 = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.").parse().expect("Failed to parse lifetime value");
    let end_time = Timestamp::new(false);

    println!("{}", boagent_url);

    let query = query_boagent(
        boagent_url,
        start_time,
        end_time,
        true,
        &location,
        lifetime.into(),
    ).await;
    println!("{:?}", query);
    Ok("Inserted data".to_string())
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use database::timestamp;
    use predicates::prelude::*;
    use std::{io::Read, process::Command};
    use tokio::time::{self, Duration};

    #[test]
    #[ignore]
    fn it_prints_process_pid() {
        // Does not presently capture output to buffer
        let mut carenaged = Command::new("../target/debug/carenaged")
            .spawn()
            .expect("Failed to execute carenaged");

        let start_daemon_message = "Started carenage daemon with PID".to_string();
        let carenaged_stdout = carenaged.stdout.take();
        let mut carenage_buffer = String::new();
        let _ = carenaged_stdout
            .expect("Failed to read output")
            .read_to_string(&mut carenage_buffer);

        let _ = carenaged.kill();
        let predicate_fn = predicate::str::contains(start_daemon_message);
        assert_eq!(true, predicate_fn.eval(&carenage_buffer));
    }

    #[tokio::test]
    #[ignore]
    async fn it_queries_boagent_and_insert_data_in_database_every_interval_of_given_timestamp() {
        let start_time = timestamp::Timestamp::new(false);
        let time_step = time::interval(Duration::from_secs(5));
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 3000,
            ..Default::default()
        };
        let mut boagent_server = Server::new_with_opts_async(opts).await;

        let mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={}", start_time).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        println!("{:?}", mock);

        let boagent_query = query_and_insert_data(time_step, start_time).await;
        println!("{:?}", boagent_query);

        mock.assert_async().await;

        assert_eq!(boagent_query.is_ok(), true);
        assert_eq!(boagent_query.unwrap(), "Inserted data");
    }
} */
