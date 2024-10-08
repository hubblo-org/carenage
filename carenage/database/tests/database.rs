use std::fs::File;
use std::io::BufReader;

use chrono::{Duration, Local};
use database::boagent::{deserialize_boagent_json, query_boagent, HardwareData};
use database::database::{
    format_hardware_data, format_process_metadata, get_db_connection_pool, get_project_id,
    insert_device_metadata, insert_dimension_table_metadata, insert_event_data, insert_metrics,
    insert_process_metadata, update_stop_date,
};
use database::event::{Event, EventType};
use database::metrics::Metrics;
use database::timestamp::Timestamp;
use dotenv::var;
use mockito::{Matcher, Server};
use serde_json::json;
use sqlx::{PgPool, Row};

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
        insert_dimension_table_metadata(db_connection, "projects", project_metadata.clone()).await;

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
        let insert_query =
            insert_dimension_table_metadata(db_connection, table, dimension_table_metadata.clone())
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

    let process_metadata = json!({
        "exe": "/snap/firefox/4336/usr/lib/firefox/firefox",
        "cmdline": "/snap/firefox/4336/usr/lib/firefox/firefox-contentproc-childID58-isForBrowser-prefsLen32076-prefMapSize244787-jsInitLen231800-parentBuildID20240527194810-greomni/snap/firefox/4336/
    usr/lib/firefox/omni.ja-appomni/snap/firefox/4336/usr/lib/firefox/browser/omni.ja-appDir/snap/firefox/4336/usr/lib/firefox/browser{1e76e076-a55a-41cf-bf27-94855c01b247}3099truetab",
        "state": "running",
        "start_date": now_timestamp.to_string(),
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
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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
async fn it_formats_process_data_from_boagent_response_with_queried_pid(
    pool: PgPool,
) -> sqlx::Result<()> {
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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
    let firefox_pid: u64 = 3099;

    let process_metadata =
        format_process_metadata(deserialized_boagent_response, firefox_pid, now_timestamp);

    assert!(process_metadata.is_ok());

    let insert = insert_process_metadata(pool.acquire().await?, process_metadata.unwrap());

    assert!(insert.await.is_ok());
    Ok(())
}

#[sqlx::test(migrations = "../../db/")]
async fn it_updates_stop_date_field_in_project_row(pool: PgPool) -> sqlx::Result<()> {
    let now_timestamp = Local::now();

    let project_metadata = json!({
        "name": "my_web_application",
        "start_date": now_timestamp.to_string(),
    });

    let _insert_query = insert_dimension_table_metadata(
        pool.acquire().await?,
        "projects",
        project_metadata.clone(),
    )
    .await;

    let project_id =
        get_project_id(pool.acquire().await?, "my_web_application".to_string()).await?;

    let stop_date_timestamp = Local::now().to_string();
    let update_query = update_stop_date(
        pool.acquire().await?,
        "projects",
        project_id,
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

    Ok(())
}
#[sqlx::test(fixtures("../fixtures/dimensions.sql"))]
async fn it_inserts_foreign_keys_events_table(pool: PgPool) -> sqlx::Result<()> {
    let connection = pool.acquire().await?;

    let query = sqlx::query("SELECT * FROM project_ids()")
        .fetch_all(&mut connection.detach())
        .await?;

    let vec_ids: Vec<uuid::Uuid> = query[0].get("project_ids");

    let event = Event {
        project_id: vec_ids[0],
        workflow_id: vec_ids[1],
        pipeline_id: vec_ids[2],
        job_id: vec_ids[3],
        run_id: vec_ids[4],
        task_id: vec_ids[5],
        process_id: vec_ids[6],
        device_id: vec_ids[7],
        event_type: EventType::Regular,
    };

    let another_connection = pool.acquire().await?;

    let insert_event = insert_event_data(another_connection, event).await;

    assert!(insert_event.is_ok());

    Ok(())
}
#[sqlx::test]
async fn it_builds_metrics_from_json_values() {
    let process_data = json!({
        "pid": 6042,
        "process_embedded_impacts": {
            "process_cpu_embedded_impact_values": {
                "gwp_cpu_average_impact": 0.38336191697478994,
                "adp_cpu_average_impact": 0.00039112499579352936,
                "pe_cpu_average_impact": 5.750428754621844,
                "gwp_cpu_max_impact": 0.9255784800000001,
                "adp_cpu_max_impact": 0.00094432144422,
                "pe_cpu_max_impact": 13.883677200000001,
                "gwp_cpu_min_impact": 0.0049683272,
                "adp_cpu_min_impact": 0.0000050689358258,
                "pe_cpu_min_impact": 0.074524908
            },
            "process_ram_embedded_impact_values": {
                "gwp_ram_average_impact": 6.628147200042126,
                "adp_ram_average_impact": 0.0003516976065328474,
                "pe_ram_average_impact": 81.16098612296481,
                "gwp_ram_max_impact": 13.492131233215332,
                "adp_ram_max_impact": 0.0007159090042114257,
                "pe_ram_max_impact": 165.20977020263672,
                "gwp_ram_min_impact": 0,
                "adp_ram_min_impact": 0,
                "pe_ram_min_impact": 0
            },
            "process_ssd_embedded_impact_values": {
                "gwp_ssd_average_impact": 0.0000021533829645868956,
                "adp_ssd_average_impact": 7.321502079595447e-11,
                "pe_ssd_average_impact": 0.00002584059557504275,
                "gwp_ssd_max_impact": 0.0003843788591787609,
                "adp_ssd_max_impact": 1.3068881212077872e-8,
                "pe_ssd_max_impact": 0.004612546310145131,
                "gwp_ssd_min_impact": 0,
                "adp_ssd_min_impact": 0,
                "pe_ssd_min_impact": 0
            },
            "process_hdd_embedded_impact_values": {
                "gwp_hdd_average_impact": 0.0000021533829645868956,
                "adp_hdd_average_impact": 7.321502079595447e-11,
                "pe_hdd_average_impact": 0.00002584059557504275,
                "gwp_hdd_max_impact": 0.0003843788591787609,
                "adp_hdd_max_impact": 1.3068881212077872e-8,
                "pe_hdd_max_impact": 0.004612546310145131,
                "gwp_hdd_min_impact": 0,
                "adp_hdd_min_impact": 0,
                "pe_hdd_min_impact": 0
            }
        }
    });
    let now_timestamp = Timestamp::ISO8601(Some(Local::now()));
    let now_timestamp_minus_one_minute =
        Timestamp::ISO8601(Some(Local::now() - Duration::minutes(1)));

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
        .with_body_from_file("/home/repair/gitlab/carenage/carenage/mocks/query_boagent_response_before_process_embedded_impacts.json")
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

    let metrics = Metrics::build(process_data, deserialized_boagent_response);

    assert!(metrics.is_ok());
}

/*
#[sqlx::test(fixtures("../fixtures/events.sql"))]
async fn it_inserts_metrics_for_an_event_into_metrics_table(pool: PgPool) -> sqlx::Result<()> {
    let connection = pool.acquire().await?;

    let query = sqlx::query("SELECT * FROM event_id()")
        .fetch_one(&mut connection.detach())
        .await?;

    let event_id: uuid::Uuid = query.get("event_id");

    let another_connection = pool.acquire().await?;

    let process_file = File::open("../mocks/process6042.json").unwrap();
    let reader = BufReader::new(process_file);

    // let metrics = Metrics {}

    // println!("{:?}", metrics);

    /*
        let insert_metrics = insert_metrics(event_id, another_connection, metrics).await;

        assert!(insert_metrics.is_ok());

    */
    Ok(())
}
*/
