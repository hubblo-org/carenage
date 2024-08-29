use crate::timestamp::Timestamp;
use chrono::{DateTime, Local};
use dotenv::{from_path, var};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::value::Number;
use serde_json::{json, Error, Value};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgQueryResult;
use sqlx::types::uuid;
use sqlx::Row;
use sqlx::{PgPool, Postgres};
use std::fmt::{Display, Formatter};
use std::path::Path;

pub mod timestamp;

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    boagent_url: String,
    start_time: Timestamp,
    end_time: Timestamp,
    fetch_hardware: HardwareData,
    location: String,
    lifetime: i16,
) -> Result<Response, reqwest::Error> {
    let mut query_parameters = vec![];

    match start_time {
        Timestamp::UnixTimestamp(start_time) => {
            query_parameters.push(("start_time", start_time.unwrap_or(0).to_string()))
        }
        Timestamp::ISO8601Timestamp(start_time) => {
            query_parameters.push(("start_time", start_time.unwrap_or(Local::now()).to_string()))
        }
    }
    match end_time {
        Timestamp::UnixTimestamp(end_time) => {
            query_parameters.push(("end_time", end_time.unwrap_or(0).to_string()))
        }
        Timestamp::ISO8601Timestamp(end_time) => {
            query_parameters.push(("end_time", end_time.unwrap_or(Local::now()).to_string()))
        }
    }
    query_parameters.push(("verbose", "true".to_string()));
    query_parameters.push(("location", location.to_string()));
    query_parameters.push(("measure_power", "true".to_string()));
    query_parameters.push(("lifetime", lifetime.to_string()));
    query_parameters.push(("fetch_hardware", fetch_hardware.to_string()));

    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);

    client.get(base_url).query(&query_parameters).send().await
}

pub async fn deserialize_boagent_json(boagent_response: Response) -> Result<Value, Error> {
    let deserialized_boagent_json = serde_json::from_value(boagent_response.json().await.unwrap())?;

    Ok(deserialized_boagent_json)
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

pub async fn insert_dimension_table_metadata(
    database_connection: PoolConnection<Postgres>,
    table: &str,
    data: Value,
) -> Result<PgQueryResult, sqlx::Error> {
    let name = data["name"].as_str();
    let start_date = data
        .get("start_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string.");

    let start_timestamptz = to_datetime_local(start_date);
    let mut connection = database_connection.detach();

    let insert_query = format!("INSERT INTO {} (name, start_date) VALUES ($1, $2)", table);
    sqlx::query(&insert_query)
        .bind(name)
        .bind(start_timestamptz)
        .execute(&mut connection)
        .await
}

pub async fn insert_process_metadata(
    database_connection: PoolConnection<Postgres>,
    process_data: Value,
) -> Result<PgQueryResult, sqlx::Error> {
    let process_exe = process_data["exe"].as_str();
    let process_cmdline = process_data["cmdline"].as_str();
    let process_state = process_data["state"].as_str();
    let process_start_date = process_data
        .get("start_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");
    let process_stop_date = process_data
        .get("stop_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");

    let start_timestamptz = to_datetime_local(process_start_date);
    let stop_timestamptz = to_datetime_local(process_stop_date);

    let mut connection = database_connection.detach();

    let insert_query = "INSERT INTO processes (exe, cmdline, state, start_date, stop_date) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(insert_query)
        .bind(process_exe)
        .bind(process_cmdline)
        .bind(process_state)
        .bind(start_timestamptz)
        .bind(stop_timestamptz)
        .execute(&mut connection)
        .await
}

pub async fn insert_device_metadata(
    database_connection: PoolConnection<Postgres>,
    device_data: Value,
) -> Result<(), sqlx::Error> {
    let device_name = device_data["device"]["name"].as_str();
    let device_lifetime = device_data["device"]["lifetime"].as_i64();
    let device_location = device_data["device"]["location"].as_str();
    let components = device_data["components"]
        .as_array()
        .expect("Unable to read JSON Array.");

    let mut connection = database_connection.detach();

    let formatted_query =
        "INSERT INTO devices (name, lifetime, location) VALUES ($1, $2, $3) RETURNING device_id";
    let insert_device_data_query = sqlx::query(formatted_query)
        .bind(device_name)
        .bind(device_lifetime)
        .bind(device_location)
        .fetch_one(&mut connection)
        .await?;

    let device_id: uuid::Uuid = insert_device_data_query.get("device_id");
    let formatted_query = "INSERT INTO components (device_id, name, model, manufacturer) VALUES ($1, $2, $3, $4) RETURNING component_id";
    for component in components {
        let insert_component_data_query = sqlx::query(formatted_query)
            .bind(device_id)
            .bind(component["name"].as_str())
            .bind(component["model"].as_str())
            .bind(component["manufacturer"].as_str())
            .fetch_one(&mut connection)
            .await?;

        let component_id: uuid::Uuid = insert_component_data_query.get("component_id");

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

    Ok(())
}

pub async fn update_project_data(
    database_connection: PoolConnection<Postgres>,
    project_name: String,
    stop_date: &str,
) -> Result<(), sqlx::Error> {
    let mut connection = database_connection.detach();

    let stop_timestamptz = to_datetime_local(stop_date);
    let formatted_query = "UPDATE projects SET stop_date = ($1) WHERE name = ($2)";
    sqlx::query(formatted_query)
        .bind(stop_timestamptz)
        .bind(project_name)
        .execute(&mut connection)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};
    use dotenv::var;
    use mockito::{Matcher, Server};
    use serde_json::json;
    use sqlx::PgPool;
    use std::{io::Write, time::SystemTime};

    #[sqlx::test]
    async fn it_queries_boagent_with_success_with_needed_query_paramaters() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={}", now_timestamp_minus_one_minute)),
                Matcher::Regex(format!("end_time={}", now_timestamp)),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        mock.assert_async().await;

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    fn it_queries_boagent_with_success_with_unspecified_timestamps() {
        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex("start_time=0".into()),
                Matcher::Regex("end_time=0".into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(None),
            Timestamp::UnixTimestamp(None),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    fn it_queries_boagent_with_success_with_iso_8601_timestamps() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[sqlx::test]
    async fn it_sends_an_error_when_it_fails_to_send_a_request_to_boagent() {
        let url = "http://url.will.fail".to_string();

        let response = query_boagent(
            url,
            Timestamp::ISO8601Timestamp(None),
            Timestamp::ISO8601Timestamp(None),
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await;

        assert!(response.is_err());
    }

    #[sqlx::test]
    async fn it_deserializes_json_from_boagent_response() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .with_body_from_file("../mocks/boagent_response.json")
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        let deserialized_json_result = deserialize_boagent_json(response).await;

        assert!(deserialized_json_result
            .as_ref()
            .is_ok_and(|response| response.is_object()));
        assert!(
            deserialized_json_result.as_ref().unwrap()["raw_data"]["hardware_data"].is_object()
        );
        assert!(
            deserialized_json_result.as_ref().unwrap()["raw_data"]["boaviztapi_data"].is_object()
        );
        assert!(deserialized_json_result.as_ref().unwrap()["raw_data"]["power_data"].is_object());
    }

    #[sqlx::test(migrations = "../../db/")]
    async fn it_inserts_valid_data_in_projects_table_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Local::now();

        let project_metadata = json!({
            "name": "my_web_application",
            "start_date": now_timestamp.to_string(),
        });

        let db_connection = pool.acquire().await?;

        let insert_query =
            insert_dimension_table_metadata(db_connection, "projects", project_metadata).await;

        assert!(insert_query.is_ok());
        assert_eq!(insert_query.unwrap().rows_affected(), 1);
        Ok(())
    }

    #[sqlx::test(migrations = "../../db/")]
    async fn it_inserts_valid_data_for_several_dimension_tables_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Local::now();

        let dimension_tables = vec![
            "projects",
            "repositories",
            "workflows",
            "pipelines",
            "runs",
            "jobs",
            "tasks",
            "containers",
        ];

        let dimension_table_metadata = json!({
            "name": "dimension_table_metadata",
            "start_date": now_timestamp.to_string(),
        });

        for table in dimension_tables {
            let db_connection = pool.acquire().await?;
            let insert_query = insert_dimension_table_metadata(
                db_connection,
                table,
                dimension_table_metadata.clone(),
            )
            .await;
            assert!(insert_query.is_ok());
            assert_eq!(insert_query.unwrap().rows_affected(), 1);
        }

        Ok(())
    }
    #[sqlx::test(migrations = "../../db/")]
    async fn it_inserts_valid_data_for_the_processes_dimension_table_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Local::now();
        let later_timestamp = Local::now() + Duration::weeks(4);

        let process_metadata = json!({
            "exe": "/snap/firefox/4336/usr/lib/firefox/firefox",
            "cmdline": "/snap/firefox/4336/usr/lib/firefox/firefox-contentproc-childID58-isForBrowser-prefsLen32076-prefMapSize244787-jsInitLen231800-parentBuildID20240527194810-greomni/snap/firefox/4336/
        usr/lib/firefox/omni.ja-appomni/snap/firefox/4336/usr/lib/firefox/browser/omni.ja-appDir/snap/firefox/4336/usr/lib/firefox/browser{1e76e076-a55a-41cf-bf27-94855c01b247}3099truetab",
            "state": "running",
            "start_date": now_timestamp.to_string(),
            "stop_date": later_timestamp.to_string(),
        });

        let db_connection = pool.acquire().await?;

        let insert_query = insert_process_metadata(db_connection, process_metadata).await;

        assert!(insert_query.is_ok());
        assert_eq!(insert_query.unwrap().rows_affected(), 1);
        Ok(())
    }

    #[sqlx::test(migrations = "../../db/")]
    async fn it_inserts_valid_data_for_the_devices_components_and_components_characteristics_dimensions_tables_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let device_metadata = json!({
          "device": {
            "name": "dell r740",
            "lifetime": 5,
            "location": "FRA"
        },
          "components": [{
                    "name": "cpu",
                    "model": "Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz",
                    "manufacturer": "Inter Corp.",
                    "characteristics": [{
                      "name": "core_unit", "value": 4}]},
                    {
                    "name": "ram",
                    "model": "not implemented yet",
                    "manufacturer": "Inter Corp.",
                    "characteristics": [{"name": "capacity", "value": 8}]
                },
                    {
                    "name": "disk",
                    "model": "not implemented yet",
                    "manufacturer": "toshiba",
                "characteristics": [{
                    "name": "type",
                    "value": "ssd"}, {"name": "capacity", "value": 238}
              ]}]
        });

        let db_connection = pool.acquire().await?;

        let insert_query = insert_device_metadata(db_connection, device_metadata).await;

        assert!(insert_query.is_ok());
        Ok(())
    }

    #[sqlx::test]
    async fn it_formats_hardware_data_from_boagent_to_wanted_database_fields() {
        let now_timestamp = Timestamp::ISO8601Timestamp(Some(Local::now()));
        let now_timestamp_minus_one_minute =
            Timestamp::ISO8601Timestamp(Some(Local::now() - Duration::minutes(1)));

        let mut boagent_server = Server::new_async().await;

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".to_string(),
                    now_timestamp_minus_one_minute.to_string(),
                ),
                Matcher::UrlEncoded("end_time".to_string(), now_timestamp.to_string()),
                Matcher::UrlEncoded("verbose".to_string(), "true".to_string()),
                Matcher::UrlEncoded("location".to_string(), "FRA".to_string()),
                Matcher::UrlEncoded("measure_power".to_string(), "true".to_string()),
                Matcher::UrlEncoded("lifetime".to_string(), "5".to_string()),
                Matcher::UrlEncoded("fetch_hardware".to_string(), "true".to_string()),
            ]))
            .with_status(200)
            .with_body_from_file("../mocks/boagent_response.json")
            .create_async()
            .await;

        let response = query_boagent(
            url,
            now_timestamp_minus_one_minute,
            now_timestamp,
            HardwareData::Inspect,
            "FRA".to_string(),
            5,
        )
        .await
        .unwrap();

        let deserialized_boagent_response = deserialize_boagent_json(response).await.unwrap();
        let location = "FRA".to_string();
        let lifetime = 5;
        let device_name = "dell r740".to_string();

        let hardware_data = format_hardware_data(
            deserialized_boagent_response,
            device_name,
            location,
            lifetime,
        )
        .unwrap();

        let device = hardware_data["device"].clone();
        let cpu = hardware_data["components"][0].clone();
        let ram = hardware_data["components"][1].clone();
        let disk = hardware_data["components"][3].clone();

        assert_eq!(device["name"], "dell r740");
        assert_eq!(device["location"], "FRA");
        assert_eq!(device["lifetime"], 5);
        assert_eq!(cpu["name"], "cpu");
        assert_eq!(ram["name"], "ram");
        assert_eq!(ram["model"].as_str(), Some("not implemented"));
        assert_eq!(ram["characteristics"][0]["value"], 4);
        assert_eq!(disk["name"], "disk");
        assert_eq!(disk["characteristics"][0]["value"], 238);
    }

    #[sqlx::test(migrations = "../../db/")]
    async fn it_updates_stop_date_field_in_project_row(pool: PgPool) -> sqlx::Result<()> {
        let now_timestamp = Local::now();

        let project_metadata = json!({
            "name": "my_web_application",
            "start_date": now_timestamp.to_string(),
        });

        let _insert_query =
            insert_dimension_table_metadata(pool.acquire().await?, "projects", project_metadata)
                .await;

        let stop_date_timestamp = Local::now().to_string();
        let update_query = update_project_data(
            pool.acquire().await?,
            "my_web_application".to_string(),
            stop_date_timestamp.as_str(),
        )
        .await;
        assert!(update_query.is_ok());
        Ok(())
    }

    #[sqlx::test]
    async fn it_acquires_a_connection_to_the_database() {
        let database_url = var("DATABASE_URL").expect("Failed to get DATABASE_URL");

        let db_connect = get_db_connection_pool(database_url)
            .await
            .unwrap()
            .acquire()
            .await;

        assert!(db_connect.is_ok());
    }

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
