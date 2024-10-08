use crate::event::Event;
use crate::metrics::{Metrics, ProcessEmbeddedImpacts};
use crate::timestamp::Timestamp;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::value::Number;
use serde_json::{json, Error, Value};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgRow;
use sqlx::types::uuid::Uuid;
use sqlx::Row;
use sqlx::{PgPool, Postgres};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
// https://serde.rs/enum-representations.html#untagged
#[serde(untagged)]
pub enum CharacteristicValue {
    StringValue(String),
    NumericValue(Number),
}

#[derive(Serialize, Deserialize)]
struct Device {
    name: String,
    location: String,
    lifetime: Number,
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    name: String,
    model: String,
    manufacturer: String,
    characteristics: Vec<ComponentCharacteristic>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComponentCharacteristic {
    name: String,
    value: CharacteristicValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    exe: String,
    cmdline: String,
    state: String,
    start_date: String,
}

#[derive(Copy, Clone)]
pub struct Ids {
    pub project_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub device_id: Uuid,
}

pub async fn get_db_connection_pool(database_url: String) -> Result<PgPool, sqlx::Error> {
    let connection_pool = PgPool::connect(database_url.as_str());

    connection_pool.await
}

pub fn to_datetime_local(timestamp_str: &str) -> chrono::DateTime<Local> {
    DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.9f %:z")
        .expect("It should be a parsable string to be converted to an ISO8601 timestamp with local timezone.").into()
}

pub fn format_hardware_data(
    deserialized_boagent_response: Value,
    device_name: String,
    location: String,
    lifetime: i16,
) -> Result<Value, Error> {
    let hardware_data = &deserialized_boagent_response["raw_data"]["hardware_data"];

    let device = Device {
        name: device_name,
        location,
        lifetime: lifetime.into(),
    };

    let mut components = vec![];

    let cpus = hardware_data["cpus"]
        .as_array()
        .expect("Unable to parse CPUs JSON array from Boagent.")
        .iter();
    let rams = hardware_data["rams"]
        .as_array()
        .expect("Unable to parse RAM JSON array from Boagent.")
        .iter();
    let disks = hardware_data["disks"]
        .as_array()
        .expect("Unable to parse Disks JSON array from Boagent.")
        .iter();

    let cpu_components_iter = cpus.map(|cpu| {
        let core_units = ComponentCharacteristic {
            name: "core_units".to_string(),
            value: CharacteristicValue::NumericValue(
                cpu["core_units"]
                    .as_number()
                    .expect("Unable to convert CPU core_units to an integer.")
                    .clone(),
            ),
        };
        Component {
            name: "cpu".to_string(),
            model: cpu["name"].to_string(),
            manufacturer: cpu["manufacturer"].to_string(),
            characteristics: vec![core_units],
        }
    });

    components.extend(cpu_components_iter);

    let ram_components_iter = rams.map(|ram| {
        let capacity = ComponentCharacteristic {
            name: "capacity".to_string(),
            value: CharacteristicValue::NumericValue(
                ram["capacity"]
                    .as_number()
                    .expect("Unable to convert RAM capacity to an integer.")
                    .clone(),
            ),
        };
        Component {
            name: "ram".to_string(),
            model: "not implemented".to_string(),
            manufacturer: ram["manufacturer"].to_string(),
            characteristics: vec![capacity],
        }
    });

    components.extend(ram_components_iter);

    let disk_components_iter = disks.map(|disk| {
        let capacity = ComponentCharacteristic {
            name: "capacity".to_string(),
            value: CharacteristicValue::NumericValue(
                disk["capacity"]
                    .as_number()
                    .expect("Unable to convert disk capacity to an integer.")
                    .clone(),
            ),
        };
        let disk_type = ComponentCharacteristic {
            name: "type".to_string(),
            value: CharacteristicValue::StringValue(disk["type"].to_string()),
        };
        Component {
            name: "disk".to_string(),
            model: "not implemented".to_string(),
            manufacturer: disk["manufacturer"].to_string(),
            characteristics: vec![capacity, disk_type],
        }
    });

    components.extend(disk_components_iter);

    let formatted_hardware_data = json!({"device": device, "components": components});
    Ok(formatted_hardware_data)
}

pub fn format_process_metadata(
    deserialized_boagent_response: Value,
    pid: u64,
    start_timestamp: Timestamp,
) -> Result<Value, Error> {
    let last_timestamp = deserialized_boagent_response["raw_data"]["power_data"]["raw_data"]
        .as_array()
        .expect("Boagent response should be parsable")
        .last()
        .expect("Last data item from Boagent should be parsable.");

    let processes = last_timestamp["consumers"]
        .as_array()
        .expect("Last data item from Boagent should contain information on processes.")
        .iter();

    let process: Vec<Process> = processes
        .filter(|process| process["pid"] == pid)
        .map(|process| Process {
            exe: process["exe"].to_string(),
            cmdline: process["cmdline"].to_string(),
            state: "running".to_string(),
            start_date: start_timestamp.to_string(),
        })
        .collect();

    Ok(json!(process[0]))
}

pub async fn insert_dimension_table_metadata(
    database_connection: PoolConnection<Postgres>,
    table: &str,
    data: Value,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let name = data["name"].as_str();
    let start_date = data
        .get("start_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string.");

    let start_timestamptz = to_datetime_local(start_date);
    let mut connection = database_connection.detach();

    let insert_query = format!(
        "INSERT INTO {} (name, start_date) VALUES ($1, $2) RETURNING *",
        table
    );

    let rows = sqlx::query(&insert_query)
        .bind(name)
        .bind(start_timestamptz)
        .fetch_all(&mut connection)
        .await?;
    Ok(rows)
}

pub async fn insert_process_metadata(
    database_connection: PoolConnection<Postgres>,
    process_data: Value,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let process_exe = process_data["exe"].as_str();
    let process_cmdline = process_data["cmdline"].as_str();
    let process_state = process_data["state"].as_str();
    let process_start_date = process_data
        .get("start_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");

    let start_timestamptz = to_datetime_local(process_start_date);

    let mut connection = database_connection.detach();

    let insert_query = "INSERT INTO processes (exe, cmdline, state, start_date) VALUES ($1, $2, $3, $4) RETURNING *";

    let process_rows = sqlx::query(insert_query)
        .bind(process_exe)
        .bind(process_cmdline)
        .bind(process_state)
        .bind(start_timestamptz)
        .fetch_all(&mut connection)
        .await?;

    Ok(process_rows)
}

pub async fn insert_device_metadata(
    database_connection: PoolConnection<Postgres>,
    device_data: Value,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let device_name = device_data["device"]["name"].as_str();
    let device_lifetime = device_data["device"]["lifetime"].as_i64();
    let device_location = device_data["device"]["location"].as_str();
    let components = device_data["components"]
        .as_array()
        .expect("Unable to read JSON Array.");

    let mut connection = database_connection.detach();

    let formatted_query =
        "INSERT INTO devices (name, lifetime, location) VALUES ($1, $2, $3) RETURNING *";
    let device_rows = sqlx::query(formatted_query)
        .bind(device_name)
        .bind(device_lifetime)
        .bind(device_location)
        .fetch_all(&mut connection)
        .await?;

    let device_id: uuid::Uuid = device_rows[0].get("id");
    let formatted_query = "INSERT INTO components (device_id, name, model, manufacturer) VALUES ($1, $2, $3, $4) RETURNING id";
    for component in components {
        let insert_component_data_query = sqlx::query(formatted_query)
            .bind(device_id)
            .bind(component["name"].as_str())
            .bind(component["model"].as_str())
            .bind(component["manufacturer"].as_str())
            .fetch_one(&mut connection)
            .await?;

        let component_id: uuid::Uuid = insert_component_data_query.get("id");

        let component_characteristics = component["characteristics"]
            .as_array()
            .expect("Unable to read JSON Array.");

        for component_characteristic in component_characteristics {
            let formatted_query = "INSERT INTO component_characteristic (component_id, name, value) VALUES ($1, $2, $3)";

            sqlx::query(formatted_query)
                .bind(component_id)
                .bind(component_characteristic["name"].as_str())
                .bind(component_characteristic["value"].as_str())
                .execute(&mut connection)
                .await?;
        }
    }

    Ok(device_rows)
}

pub async fn insert_event_data(
    database_connection: PoolConnection<Postgres>,
    event: Event,
) -> Result<PgRow, sqlx::Error> {
    let mut connection = database_connection.detach();

    let timestamptz = Local::now();
    let formatted_query = "INSERT INTO events (timestamp, project_id, workflow_id, pipeline_id, job_id, run_id, task_id, process_id, device_id, event_type) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
    RETURNING id";

    let event = sqlx::query(formatted_query)
        .bind(timestamptz)
        .bind(event.project_id)
        .bind(event.workflow_id)
        .bind(event.pipeline_id)
        .bind(event.job_id)
        .bind(event.run_id)
        .bind(event.task_id)
        .bind(event.process_id)
        .bind(event.device_id)
        .bind(event.event_type)
        .fetch_one(&mut connection)
        .await?;

    Ok(event)
}

pub async fn insert_metrics(
    event_id: Uuid,
    database_connection: PoolConnection<Postgres>,
    metrics: Metrics,
) -> Result<(), sqlx::Error> {
    let mut connection = database_connection.detach();

    let formatted_query = "INSERT INTO METRICS (event_id, metric, value) VALUES ($1, $2, $3)";

    //let iterable_metrics = serde_json::to_value(metrics.process_embedded_impacts);


    /*
        sqlx::query(formatted_query)
            .bind(event_id)
            .bind(key.get("{key}"))
            .execute(&mut connection)
            .await?; */
    Ok(())
}

pub async fn update_stop_date(
    database_connection: PoolConnection<Postgres>,
    table_name: &str,
    row_id: uuid::Uuid,
    stop_date: &str,
) -> Result<(), sqlx::Error> {
    let mut connection = database_connection.detach();

    let stop_timestamptz = to_datetime_local(stop_date);
    let formatted_query = format!("UPDATE {} SET stop_date = ($1) WHERE id = ($2)", table_name);
    sqlx::query(&formatted_query)
        .bind(stop_timestamptz)
        .bind(row_id)
        .execute(&mut connection)
        .await?;

    Ok(())
}

pub async fn get_project_id(
    database_connection: PoolConnection<Postgres>,
    project_name: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let mut connection = database_connection.detach();

    let formatted_query = "SELECT id FROM PROJECTS WHERE name = ($1)";

    let project_row = sqlx::query(formatted_query)
        .bind(project_name)
        .fetch_one(&mut connection)
        .await?;
    Ok(project_row.get("id"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boagent::Config;
    use chrono::Local;
    use std::io::Write;

    #[test]
    fn it_checks_that_all_needed_configuration_details_are_set() -> std::io::Result<()> {
        let current_dir = std::env::current_dir().unwrap();
        let config_path = current_dir.join("../mocks/").canonicalize().unwrap();
        let env_file = config_path.join(".env");
        let mut config_file =
            std::fs::File::create(env_file.clone()).expect("Failed to create env file for testing");
        config_file.write_all(
            b"DATABASE_URL='postgres://carenage:password@localhost:5432/carenage'
PROJECT_NAME='carenage_webapp'
BOAGENT_URL='http://localhost:8000'
LOCATION='FRA'
LIFETIME=5
",
        )?;
        let config_check = Config::check_configuration(env_file.as_path());

        assert!(config_check.is_ok());
        Ok(())
    }

    #[test]
    fn it_converts_a_parsable_string_containing_an_iso8601_timestamp_to_a_datetime_with_local_offset(
    ) {
        let dt_local_timestamp = Local::now();
        let converted_string = to_datetime_local(dt_local_timestamp.to_string().as_str());
        assert_eq!(dt_local_timestamp, converted_string);
    }
}
