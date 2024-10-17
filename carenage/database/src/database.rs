use crate::metrics::Metrics;
use crate::tables::{
    CharacteristicValue, ComponentBuilder, ComponentCharacteristicBuilder,
    DeviceBuilder, Process, ProcessBuilder,
};
use crate::timestamp::Timestamp;
use chrono::{DateTime, Local};
use serde_json::{json, Error, Value};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgRow;
use sqlx::types::uuid::Uuid;
use sqlx::Row;
use sqlx::{PgPool, Postgres};

#[derive(Copy, Clone)]
pub struct Ids {
    pub project_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub device_id: Uuid,
    pub process_id: Uuid,
}

pub async fn get_db_connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let connection_pool = PgPool::connect(database_url);

    connection_pool.await
}

pub fn to_datetime_local(timestamp_str: &str) -> chrono::DateTime<Local> {
    DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.9f %:z")
        .expect("It should be a parsable string to be converted to an ISO8601 timestamp with local timezone.").into()
}

pub fn format_hardware_data(
    deserialized_boagent_response: Value,
    device_name: &str,
    location: &str,
    lifetime: i16,
) -> Result<Value, Error> {
    let hardware_data = &deserialized_boagent_response["raw_data"]["hardware_data"];

    let device = DeviceBuilder::new(device_name, location, lifetime).build();

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
        let core_units = ComponentCharacteristicBuilder::new(
            "core_units",
            CharacteristicValue::NumericValue(
                cpu["core_units"]
                    .as_number()
                    .expect("Unable to convert CPU core_units to an integer.")
                    .clone(),
            ),
        )
        .build();
        ComponentBuilder::new(
            "cpu",
            cpu["name"].as_str().unwrap(),
            cpu["manufacturer"].as_str().unwrap(),
            vec![core_units],
        )
        .build()
    });

    components.extend(cpu_components_iter);

    let ram_components_iter = rams.map(|ram| {
        let capacity = ComponentCharacteristicBuilder::new(
            "capacity",
            CharacteristicValue::NumericValue(
                ram["capacity"]
                    .as_number()
                    .expect("Unable to convert RAM capacity to an integer.")
                    .clone(),
            ),
        )
        .build();
        ComponentBuilder::new(
            "ram",
            "not implemented",
            ram["manufacturer"].as_str().unwrap(),
            vec![capacity],
        )
        .build()
    });

    components.extend(ram_components_iter);

    let disk_components_iter = disks.map(|disk| {
        let capacity = ComponentCharacteristicBuilder::new(
            "capacity",
            CharacteristicValue::NumericValue(
                disk["capacity"]
                    .as_number()
                    .expect("Unable to convert disk capacity to an integer.")
                    .clone(),
            ),
        )
        .build();
        let disk_type = ComponentCharacteristicBuilder::new(
            "type",
            CharacteristicValue::StringValue(disk["type"].to_string()),
        )
        .build();
        ComponentBuilder::new(
            "disk",
            "not implemented",
            disk["manufacturer"].as_str().unwrap(),
            vec![capacity, disk_type],
        )
        .build()
    });

    components.extend(disk_components_iter);

    let formatted_hardware_data = json!({"device": device, "components": components});
    Ok(formatted_hardware_data)
}

pub fn collect_processes(
    deserialized_boagent_response: &Value,
    start_timestamp: Timestamp,
) -> Result<Vec<Process>, Error> {
    let processes: Vec<Process> = deserialized_boagent_response["raw_data"]["power_data"]
        ["raw_data"]
        .as_array()
        .expect("Boagent response should be parsable")
        .last()
        .expect("Last data item from Boagent should be parsable.")
        .get("consumers")
        .expect("Consumers should be available from Scaphandre.")
        .as_array()
        .expect("Consumers should contain information on processes.")
        .iter()
        .map(|process| {
            ProcessBuilder::new(
                process["pid"]
                    .as_i64()
                    .expect("Process ID should be an integer.") as i32,
                process["exe"].as_str().expect("Exe should be available."),
                process["cmdline"]
                    .as_str()
                    .expect("Cmdline should be available."),
                "running",
                start_timestamp,
            )
            .build()
        })
        .collect();

    Ok(processes)
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

pub async fn insert_metrics(
    event_id: Uuid,
    database_connection: PoolConnection<Postgres>,
    metrics: Metrics,
) -> Result<(), sqlx::Error> {
    let mut connection = database_connection.detach();

    let metrics_value = serde_json::to_value(metrics).expect("Metrics should be deserializable.");
    let iterable_metrics = metrics_value
        .as_object()
        .expect("Metrics should be parsable.");

    let metric_fields: Vec<String> = iterable_metrics
        .iter()
        .map(|metric| metric.0.clone())
        .collect();

    let metric_values: Vec<f64> = iterable_metrics
        .iter()
        .map(|metric| metric.1.as_f64().unwrap())
        .collect();

    let query = "INSERT INTO METRICS (event_id, metric, value) VALUES ($1, UNNEST($2::VARCHAR(255)[]), UNNEST($3::NUMERIC[]))";

    sqlx::query(query)
        .bind(event_id)
        .bind(metric_fields)
        .bind(metric_values)
        .execute(&mut connection)
        .await?;

    println!("Inserted metrics.");
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
    project_name: &String,
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
