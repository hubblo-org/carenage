use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::value::Number;
use serde_json::{json, Error, Value};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgRow;
use sqlx::types::uuid;
use sqlx::Row;
use sqlx::{PgPool, Postgres};

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
    let process_stop_date = process_data
        .get("stop_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");

    let start_timestamptz = to_datetime_local(process_start_date);
    let stop_timestamptz = to_datetime_local(process_stop_date);

    let mut connection = database_connection.detach();

    let insert_query = "INSERT INTO processes (exe, cmdline, state, start_date, stop_date) VALUES ($1, $2, $3, $4, $5) RETURNING *";

    let process_rows = sqlx::query(insert_query)
        .bind(process_exe)
        .bind(process_cmdline)
        .bind(process_state)
        .bind(start_timestamptz)
        .bind(stop_timestamptz)
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

    let device_id: uuid::Uuid = device_rows[0].get("device_id");
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

    Ok(device_rows)
}

pub async fn update_stop_date(
    database_connection: PoolConnection<Postgres>,
    table_name: &str,
    project_name: String,
    stop_date: &str,
) -> Result<(), sqlx::Error> {
    let mut connection = database_connection.detach();

    let stop_timestamptz = to_datetime_local(stop_date);
    let formatted_query = format!(
        "UPDATE {} SET stop_date = ($1) WHERE name = ($2)",
        table_name
    );
    sqlx::query(&formatted_query)
        .bind(stop_timestamptz)
        .bind(project_name)
        .execute(&mut connection)
        .await?;

    Ok(())
}

pub async fn get_project_id(
    database_connection: PoolConnection<Postgres>,
    project_name: String,
) -> Result<PgRow, sqlx::Error> {
    let mut connection = database_connection.detach();

    let formatted_query = "SELECT id FROM PROJECTS WHERE name = ($1)";

    let project_id = sqlx::query(formatted_query)
        .bind(project_name)
        .fetch_one(&mut connection)
        .await?;
    Ok(project_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boagent::{deserialize_boagent_json, query_boagent, Config, HardwareData};
    use crate::timestamp::Timestamp;
    use chrono::{Duration, Local};
    use dotenv::var;
    use mockito::{Matcher, Server};
    use serde_json::json;
    use sqlx::PgPool;
    use std::io::Write;
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
            insert_dimension_table_metadata(db_connection, "projects", project_metadata.clone())
                .await;

        assert!(insert_query.is_ok());

        let rows = insert_query.unwrap();
        let project_name: String = rows[0].get("name");
        assert_eq!(project_name, project_metadata["name"]);
        assert_eq!(rows.len(), 1);
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
            let rows = insert_query.unwrap();
            assert_eq!(rows.len(), 1);
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

        let insert_query = insert_process_metadata(db_connection, process_metadata.clone()).await;

        assert!(insert_query.is_ok());

        let rows = insert_query.unwrap();
        let process_exe: String = rows[0].get("exe");
        assert_eq!(rows.len(), 1);
        assert_eq!(process_exe, process_metadata["exe"]);
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
        let update_query = update_stop_date(
            pool.acquire().await?,
            "projects",
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

    #[sqlx::test(migrations = "../../db/")]
    async fn it_gets_the_project_path_as_an_environment_variable_and_inserts_it_as_project_name(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Local::now();
        std::env::set_var("CI_PROJECT_PATH", "hubblo/carenage");

        let project_metadata = json!({
            "name": std::env::var("CI_PROJECT_PATH").is_ok().to_string(),
            "start_date": now_timestamp.to_string(),
        });

        let _insert_query =
            insert_dimension_table_metadata(pool.acquire().await?, "projects", project_metadata);
        Ok(())
    }

    #[sqlx::test(migrations = "../../db/")]
    async fn it_gets_project_id_from_projects_table_with_queried_project_name(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Local::now();

        let project_name = "my_web_application";

        let project_metadata = json!({
            "name": project_name,
            "start_date": now_timestamp.to_string(),
        });

        let db_connection = pool.acquire().await?;

        insert_dimension_table_metadata(db_connection, "projects", project_metadata).await?;

        let db_connection = pool.acquire().await?;

        let project_id_query = get_project_id(db_connection, project_name.to_string()).await;

        assert!(project_id_query.is_ok());

        let project_row = project_id_query.unwrap();

        assert_eq!(project_row.len(), 1);

        Ok(())
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
