use crate::timestamp::Timestamp;
use dotenv::{from_path, var};
use reqwest::{Client, Response};
use serde_json::{Error, Value};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Clone, Copy)]
pub enum HardwareData {
    Inspect,
    Ignore,
}

impl Display for HardwareData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            HardwareData::Inspect => {
                write!(f, "true")
            }
            HardwareData::Ignore => {
                write!(f, "false")
            }
        }
    }
}

pub struct Config {
    pub boagent_url: String,
    pub database_url: String,
    pub location: String,
    pub lifetime: i16,
    pub device_name: String,
    pub project_name: String,
}

impl Config {
    pub fn check_configuration(config_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
        let _load_config = from_path(config_path);
        let boagent_url = var("BOAGENT_URL").expect("BOAGENT_URL environment variable is absent. It is needed to connect to Boagent and query necessary data.");
        let project_name = var("PROJECT_NAME").expect("PROJECT_NAME environment variable is absent. It is needed to refer to the project in collected data.");
        let location = var("LOCATION").expect("LOCATION environment variable is absent. It is needed to indicate the energy mix relevant to the evaluated environmental impacts.");
        let lifetime: i16 = var("LIFETIME").expect("LIFETIME environment variable is absent. It is needed to calculate the environmental impact for the evaluated device.").parse().expect("Failed to parse lifetime value.");
        let device_name = var("DEVICE").unwrap_or("unknown".to_string());
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable is absent.");
        Ok(Config {
            boagent_url,
            project_name,
            location,
            lifetime,
            device_name,
            database_url,
        })
    }
}

pub async fn query_boagent(
    boagent_url: &String,
    start_time: Timestamp,
    end_time: Timestamp,
    fetch_hardware: HardwareData,
    location: String,
    lifetime: i16,
) -> Result<Response, reqwest::Error> {
    let query_parameters = vec![
        ("start_time", start_time.as_query_parameter()),
        ("end_time", end_time.as_query_parameter()),
        ("verbose", "true".to_string()),
        ("location", location.to_string()),
        ("measure_power", "true".to_string()),
        ("lifetime", lifetime.to_string()),
        ("fetch_hardware", fetch_hardware.to_string()),
    ];

    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);

    client.get(base_url).query(&query_parameters).send().await
}

pub async fn process_embedded_impacts(
    boagent_url: &String,
    process_id: i32,
    start_time: Timestamp,
    end_time: Timestamp,
    fetch_hardware: HardwareData,
    location: String,
    lifetime: i16,
) -> Result<Response, reqwest::Error> {
    let query_parameters = vec![
        ("process_id", process_id.to_string()),
        ("start_time", start_time.as_query_parameter()),
        ("end_time", end_time.as_query_parameter()),
        ("verbose", "true".to_string()),
        ("location", location.to_string()),
        ("measure_power", "true".to_string()),
        ("lifetime", lifetime.to_string()),
        ("fetch_hardware", fetch_hardware.to_string()),
    ];

    let client = Client::new();
    let base_url = format!("{}/process_embedded_impacts", boagent_url);

    client.get(base_url).query(&query_parameters).send().await
}

pub async fn deserialize_boagent_json(boagent_response: Response) -> Result<Value, Error> {
    let deserialized_boagent_json = serde_json::from_value(boagent_response.json().await.unwrap())?;

    Ok(deserialized_boagent_json)
}

pub fn get_processes_ids(deseriliazed_boagent_response: Value) -> Result<Vec<u64>, Error> {
    // Need to get last measured processes by Scaphandre: processes during the execution of
    // carenaged and the associated Scaphandre measurements might change, depending on the
    // configuration set for Scaphandre (ten most energy intensive processes, or something else).
    // By getting the last item in raw_data from Scaphandre, those processes will be the last
    // measured by Scaphandre.
    let last_timestamp = deseriliazed_boagent_response["raw_data"]["power_data"]["raw_data"]
        .as_array()
        .expect("Data from Scaphandre should be parsable.")
        .last()
        .expect("Last recorded data from Scaphandre should be parsable.");
    let processes = last_timestamp["consumers"]
        .as_array()
        .expect("Processes should be parsable from Scaphandre.")
        .iter();

    let processes_ids = processes
        .map(|process| {
            process
                .get("pid")
                .expect("Consumer should have a pid key.")
                .as_u64()
                .expect("Process ID returned from Scaphandre should be parsable.")
        })
        .collect();
    Ok(processes_ids)
}
