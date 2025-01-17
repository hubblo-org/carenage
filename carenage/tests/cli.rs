use assert_cmd::prelude::*;
use chrono::Utc;
use predicates::str::contains;
use std::{process::Command, time::SystemTime};

// carenage start
#[test]
fn it_accepts_start_as_a_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("start");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event"));

    Ok(())
}

#[test]
fn it_accepts_start_as_a_command_and_has_a_default_time_step_of_five_seconds(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("start");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event, time step of 5 seconds\n"));

    Ok(())
}

#[test]
fn it_accepts_start_with_a_specified_time_step_in_seconds() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("start").arg("--step").arg("10");
    cmd.assert()
        .success()
        .stdout(contains("Carenage start event, time step of 10 seconds\n"));

    Ok(())
}

#[test]
fn it_fails_when_giving_an_invalid_time_step_with_start_command(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("start").arg("--step").arg("test");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn it_prints_start_timestamp_in_iso_8601_format() -> Result<(), Box<dyn std::error::Error>> {
    // Test will probably have to be rewritten as printing the start timestamp will depend on how
    // GitHub, GitLab and other print their first timestamp in pipelines

    let start_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut cmd = Command::cargo_bin("carenage-cli")?;

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

    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("--unix").arg("start");
    cmd.assert().success().stdout(contains(start_timestamp));

    Ok(())
}

#[test]
fn it_prints_confirmation_that_it_is_the_first_time_carenage_is_used_for_a_project(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("start").arg("--init");
    cmd.assert().success().stdout(contains(
        "`init` flag set: this is the first time Carenage is used for this project.",
    ));

    Ok(())
}

// carenage stop
#[test]
fn it_accepts_stop_as_a_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("stop");
    cmd.assert()
        .success()
        .stdout(contains("Carenage stop event"));

    Ok(())
}

#[test]
fn it_prints_stop_timestamp_in_iso_8601_format() -> Result<(), Box<dyn std::error::Error>> {
    let stop_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut cmd = Command::cargo_bin("carenage-cli")?;

    cmd.arg("stop");
    cmd.assert().success().stdout(contains(stop_timestamp));

    Ok(())
}
