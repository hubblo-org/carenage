use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::blocking::{Client, Response};
use serde_json::{Error, Value};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgQueryResult;
use sqlx::types::uuid;
use sqlx::Postgres;
use sqlx::Row;
use std::fs::File;
use std::io::BufReader;

enum Timestamp {
    UnixTimestamp(Option<u64>),
    ISO8601Timestamp(DateTime<Utc>),
}

fn query_boagent(
    boagent_url: String,
    start_time: Timestamp,
    end_time: Timestamp,
    location: String,
    lifetime: u8,
) -> Result<Response, String> {
    let mut query_parameters = vec![];

    match start_time {
        Timestamp::UnixTimestamp(start_time) => {
            query_parameters.push(("start_time", start_time.unwrap_or(0).to_string()))
        }
        Timestamp::ISO8601Timestamp(start_time) => {
            query_parameters.push(("start_time", format!("{:?}", start_time)))
        }
    }
    match end_time {
        Timestamp::UnixTimestamp(end_time) => {
            query_parameters.push(("end_time", end_time.unwrap_or(0).to_string()))
        }
        Timestamp::ISO8601Timestamp(end_time) => {
            query_parameters.push(("end_time", format!("{:?}", end_time)))
        }
    }
    query_parameters.push(("verbose", "true".to_string()));
    query_parameters.push(("location", location));
    query_parameters.push(("measure_power", "true".to_string()));
    query_parameters.push(("lifetime", lifetime.to_string()));
    query_parameters.push(("fetch_hardware", "true".to_string()));

    let client = Client::new();
    let base_url = format!("{}/query", boagent_url);

    let response = client
        .get(base_url)
        .query(&query_parameters)
        .send()
        .expect("Failed to send request.");

    match response.status().as_u16() {
        200 => Ok(response),
        _ => Err("Error from Boagent.".to_string()),
    }
}

fn deserialize_boagent_json(boagent_response_json: File) -> Result<Value, Error> {
    let boagent_json_reader = BufReader::new(boagent_response_json);
    let deserialized_boagent_json = serde_json::from_reader(boagent_json_reader)?;

    Ok(deserialized_boagent_json)
}

async fn insert_dimension_table_metadata(
    database_connection: PoolConnection<Postgres>,
    table: &str,
    project_data: Value,
) -> Result<PgQueryResult, sqlx::Error> {
    let project_name = project_data["name"].as_str();
    let project_start_date = project_data
        .get("start_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");
    let project_stop_date = project_data
        .get("stop_date")
        .expect("Unable to read timestamp.")
        .as_str()
        .expect("Unable to read string");

    let start_date_timestamp =
        NaiveDateTime::parse_from_str(project_start_date, "%Y-%m-%d %H:%M:%S")
            .expect("Unable to convert to Postgres timestamp type.");
    let stop_date_timestamp = NaiveDateTime::parse_from_str(project_stop_date, "%Y-%m-%d %H:%M:%S")
        .expect("Unable to convert to Postgres timestamp type.");

    let mut connection = database_connection.detach();

    let insert_query = format!(
        "INSERT INTO {} (name, start_date, stop_date) VALUES ($1, $2, $3)",
        table
    );
    let insert_data_query = sqlx::query(&insert_query)
        .bind(project_name)
        .bind(start_date_timestamp)
        .bind(stop_date_timestamp)
        .execute(&mut connection)
        .await;

    insert_data_query
}

async fn insert_process_metadata(
    database_connection: PoolConnection<Postgres>,
    table: &str,
    process_data: Value,
) -> std::result::Result<PgQueryResult, sqlx::Error> {
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

    let start_date_timestamp =
        NaiveDateTime::parse_from_str(process_start_date, "%Y-%m-%d %H:%M:%S")
            .expect("Unable to convert to Postgres timestamp type.");
    let stop_date_timestamp = NaiveDateTime::parse_from_str(process_stop_date, "%Y-%m-%d %H:%M:%S")
        .expect("Unable to convert to Postgres timestamp type.");

    let mut connection = database_connection.detach();

    let insert_query = format!(
        "INSERT INTO {} (exe, cmdline, state, start_date, stop_date) VALUES ($1, $2, $3, $4, $5)",
        table
    );
    let insert_data_query = sqlx::query(&insert_query)
        .bind(process_exe)
        .bind(process_cmdline)
        .bind(process_state)
        .bind(start_date_timestamp)
        .bind(stop_date_timestamp)
        .execute(&mut connection)
        .await;

    insert_data_query
}

async fn insert_device_metadata(
    database_connection: PoolConnection<Postgres>,
    device_data: Value,
) -> std::result::Result<(), sqlx::Error> {
    let device_name = device_data["device"]["name"].as_str();
    let device_lifetime = device_data["device"]["lifetime"].as_i64();
    let device_location = device_data["device"]["location"].as_str();
    let components_keys = device_data["components"]
        .as_object()
        .expect("Unable to read JSON Object.")
        .keys();

    let mut connection = database_connection.detach();

    let formatted_query =
        "INSERT INTO devices (name, lifetime, location) VALUES ($1, $2, $3) RETURNING device_id";
    let insert_device_data_query = sqlx::query(&formatted_query)
        .bind(device_name)
        .bind(device_lifetime)
        .bind(device_location)
        .fetch_one(&mut connection)
        .await?;

    let device_id: uuid::Uuid = insert_device_data_query.get("device_id");
    let formatted_query = "INSERT INTO components (device_id, name, model, manufacturer) VALUES ($1, $2, $3, $4) RETURNING component_id";
    for component in components_keys {
        let insert_component_data_query = sqlx::query(&formatted_query)
            .bind(device_id)
            .bind(component)
            .bind(device_data[component]["model"].as_str())
            .bind(device_data[component]["manufacturer"].as_str())
            .fetch_one(&mut connection)
            .await?;

        let component_id: uuid::Uuid = insert_component_data_query.get("component_id");
        let component_characteristics = device_data["components"][component]["characteristics"]
            .as_object()
            .expect("Unable to read JSON Object.")
            .keys();
        for component_characteristic in component_characteristics {
            let formatted_query = "INSERT INTO component_characteristic (component_id, name, value) VALUES ($1, $2, $3)";
            let insert_component_characteristic_data_query = sqlx::query(&formatted_query)
                .bind(component_id)
                .bind(device_data[component][component_characteristic]["name"].as_str())
                .bind(device_data[component][component_characteristic]["value"].as_str())
                .execute(&mut connection)
                .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use mockito::{Matcher, Server};
    use serde_json::json;
    use sqlx::PgPool;
    use std::env::current_dir;
    use std::fs::File;
    use std::time::SystemTime;

    #[test]
    fn it_queries_boagent_with_success_with_needed_query_paramaters() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={:?}", now_timestamp_minus_one_minute).into()),
                Matcher::Regex(format!("end_time={:?}", now_timestamp).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            "FRA".to_string(),
            5,
        )
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn it_queries_boagent_with_error_if_500_http_code_received_from_boagent() {
        let now_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60;

        let error_message = "Error from Boagent.";

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={:?}", now_timestamp_minus_one_minute).into()),
                Matcher::Regex(format!("end_time={:?}", now_timestamp).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(500)
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(Some(now_timestamp_minus_one_minute)),
            Timestamp::UnixTimestamp(Some(now_timestamp)),
            "FRA".to_string(),
            5,
        );

        assert_eq!(response.is_err(), true);
        assert_eq!(response.unwrap_err(), error_message);
    }

    #[test]
    fn it_queries_boagent_with_success_with_unspecified_timestamps() {
        let mut boagent_server = Server::new();

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
            .create();

        let response = query_boagent(
            url,
            Timestamp::UnixTimestamp(None),
            Timestamp::UnixTimestamp(None),
            "FRA".to_string(),
            5,
        )
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn it_queries_boagent_with_success_with_iso_8601_timestamps() {
        let now_timestamp = Utc::now();
        let now_timestamp_minus_one_minute = now_timestamp - Duration::minutes(1);

        let mut boagent_server = Server::new();

        let url = boagent_server.url();

        let _mock = boagent_server
            .mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "start_time".into(),
                    format!("{:?}", now_timestamp_minus_one_minute),
                ),
                Matcher::UrlEncoded("end_time".into(), format!("{:?}", now_timestamp)),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into()),
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(
            url,
            Timestamp::ISO8601Timestamp(now_timestamp_minus_one_minute),
            Timestamp::ISO8601Timestamp(now_timestamp),
            "FRA".to_string(),
            5,
        )
        .unwrap();

        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn it_deserializes_json_from_boagent_response_as_a_saved_json_file() {
        let mut boagent_json_fp = current_dir().unwrap();

        boagent_json_fp.push("mocks");
        boagent_json_fp.push("boagent_response");
        boagent_json_fp.set_extension("json");

        let boagent_response_json = File::open(boagent_json_fp).unwrap();

        let deserialized_json = deserialize_boagent_json(boagent_response_json);

        assert_eq!(deserialized_json.is_ok(), true);
    }

    #[sqlx::test(migrations = "../db/")]
    async fn it_inserts_valid_data_in_projects_table_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let later_timestamp = (Utc::now() + Duration::weeks(4)).format("%Y-%m-%d %H:%M:%S");

        let project_metadata = json!({
            "name": "my_web_application",
            "start_date": now_timestamp.to_string(),
            "stop_date": later_timestamp.to_string(),
        });

        let db_connection = pool.acquire().await?;

        let insert_query =
            insert_dimension_table_metadata(db_connection, "projects", project_metadata).await;

        assert_eq!(insert_query.is_ok(), true);
        assert_eq!(insert_query.unwrap().rows_affected(), 1);
        Ok(())
    }

    #[sqlx::test(migrations = "../db/")]
    async fn it_inserts_valid_data_for_several_dimension_tables_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let later_timestamp = (Utc::now() + Duration::weeks(4)).format("%Y-%m-%d %H:%M:%S");

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
            "stop_date": later_timestamp.to_string(),
        });

        for table in dimension_tables {
            let db_connection = pool.acquire().await?;
            let insert_query = insert_dimension_table_metadata(
                db_connection,
                table,
                dimension_table_metadata.clone(),
            )
            .await;
            assert_eq!(insert_query.is_ok(), true);
            assert_eq!(insert_query.unwrap().rows_affected(), 1);
        }

        Ok(())
    }
    #[sqlx::test(migrations = "../db/")]
    async fn it_inserts_valid_data_for_the_processes_dimension_table_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let now_timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let later_timestamp = (Utc::now() + Duration::weeks(4)).format("%Y-%m-%d %H:%M:%S");

        let process_metadata = json!({
            "exe": "/snap/firefox/4336/usr/lib/firefox/firefox",
            "cmdline": "/snap/firefox/4336/usr/lib/firefox/firefox-contentproc-childID58-isForBrowser-prefsLen32076-prefMapSize244787-jsInitLen231800-parentBuildID20240527194810-greomni/snap/firefox/4336/
        usr/lib/firefox/omni.ja-appomni/snap/firefox/4336/usr/lib/firefox/browser/omni.ja-appDir/snap/firefox/4336/usr/lib/firefox/browser{1e76e076-a55a-41cf-bf27-94855c01b247}3099truetab",
            "state": "running",
            "start_date": now_timestamp.to_string(),
            "stop_date": later_timestamp.to_string(),
        });

        let db_connection = pool.acquire().await?;

        let insert_query =
            insert_process_metadata(db_connection, "processes", process_metadata).await;

        assert_eq!(insert_query.is_ok(), true);
        assert_eq!(insert_query.unwrap().rows_affected(), 1);
        Ok(())
    }

    #[sqlx::test(migrations = "../db/")]
    async fn it_inserts_valid_data_for_the_devices_components_and_components_characteristics_dimensions_tables_in_the_carenage_database(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let device_metadata = json!({
          "device": {
            "name": "dell r740",
            "lifetime": 5,
            "location": "FRA"
        },
          "components": {
            "cpu": {
                    "model": "Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz",
                    "manufacturer": "Inter Corp.",
                    "characteristics": {
                      "units": 1,
                      "core_units": 4
                    }
            },
            "ram": {
              "manufacturer": "Inter Corp.",
              "characteristics": {
                "units": 2,
                "capacity": 8
              }
            },
            "storage": {
              "manufacturer": "toshiba",
              "characteristics": {
                "type": "ssd",
                "capacity": 238
              }
            }
          }
        });

        let db_connection = pool.acquire().await?;

        let insert_query = insert_device_metadata(db_connection, device_metadata).await;

        assert_eq!(insert_query.is_ok(), true);
        Ok(())
    }
}
