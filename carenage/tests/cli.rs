use assert_cmd::prelude::*;
use chrono::Utc;
use mockito::Matcher;
use predicates::str::contains;
use std::{process::Command, time::SystemTime};

// carenage start
#[test]
fn it_accepts_start_as_a_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event"));

    Ok(())
}

#[test]
fn it_accepts_start_as_a_command_and_has_a_default_time_step_of_five_seconds(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event, time step of 5 seconds\n"));

    Ok(())
}

#[test]
fn it_accepts_start_with_a_specified_time_step_in_seconds() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start").arg("--step").arg("10");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event, time step of 10 seconds\n"));

    Ok(())
}

#[test]
fn it_fails_when_giving_an_invalid_time_step_with_start_command(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start").arg("--step").arg("test");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn it_prints_start_timestamp_in_iso_8601_format() -> Result<(), Box<dyn std::error::Error>> {
    // Test will probably have to be rewritten as printing the start timestamp will depend on how
    // GitHub, GitLab and other print their first timestamp in pipelines

    let start_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start");
    cmd.assert().success().stdout(contains(start_timestamp));

    Ok(())
}

#[test]
fn it_prints_start_timestamp_in_unix_epoch_format_in_seconds_with_given_argument(
) -> Result<(), Box<dyn std::error::Error>> {
    let start_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("--unix").arg("start");
    cmd.assert().success().stdout(contains(start_timestamp));

    Ok(())
}

#[test]
fn it_queries_boagent_when_calling_start_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    let opts = mockito::ServerOpts {
        host: "127.0.0.1",
        port: 3000,
        ..Default::default()
    };

    let mut boagent_server = mockito::Server::new_with_opts(opts);

    let mock = boagent_server
        .mock("GET", "/query")
        .match_query(Matcher::AllOf(vec![
            Matcher::Regex("verbose=true".into()),
            Matcher::Regex("location=FRA".into()),
            Matcher::Regex("measure_power=true".into()),
            Matcher::Regex("lifetime=5".into()),
            Matcher::Regex("fetch_hardware=true".into()),
        ]))
        .with_status(200)
        .create();

    cmd.arg("start");
    cmd.assert().success().stdout(contains("Queried Boagent"));

    mock.assert();
    Ok(())
}

#[test]
fn it_prints_error_message_when_it_fails_to_request_boagent(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start");
    cmd.assert()
        .success()
        .stdout(contains("Failed to query Boagent"));

    Ok(())
}

#[test]
fn it_prints_succesful_insertion_message_after_inserting_boagent_response_in_database(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("start");
    cmd.assert().success().stdout(contains(
        "Inserted project and device data in Carenage database",
    ));

    Ok(())
}

// carenage stop
#[test]
fn it_accepts_stop_as_a_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("stop");
    cmd.assert()
        .success()
        .stdout(contains("Carenage stop event"));

    Ok(())
}

#[test]
fn it_prints_stop_timestamp_in_iso_8601_format() -> Result<(), Box<dyn std::error::Error>> {
    let stop_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("stop");
    cmd.assert().success().stdout(contains(stop_timestamp));

    Ok(())
}

#[test]
fn it_queries_boagent_when_calling_stop_command_with_start_and_stop_timestamps(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage")?;

    cmd.arg("stop");
    cmd.assert()
        .success()
        .stdout(contains("Queried Boagent"))
        .stdout(contains("Carenage worked between"))
        .stdout(contains("timestamps"));

    Ok(())
}
