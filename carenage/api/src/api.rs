use axum::Extension;
use axum::{debug_handler, extract::Path, response::Json, routing::get, Router};
use database::boagent::Config;
use database::database::{get_db_connection_pool, select_metrics_from_dimension, Record};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProcessInfo {
    pub process_pid: i32,
    pub process_exe: String,
    pub process_cmdline: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessMetrics {
    pub metric_name: String,
    pub metric_values: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessRecord {
    pub process: ProcessInfo,
    pub metrics: Vec<ProcessMetrics>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub project_name: String,
    pub processes: Vec<ProcessRecord>,
}

pub struct ApiResponseBuilder(ApiResponse);

impl ApiResponseBuilder {
    pub fn new(records: &[Record], project_name: &str) -> Self {
        let mut metrics_names: Vec<String> = records
            .iter()
            .map(|record| record.metric.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        let mut processes_infos: Vec<ProcessInfo> = records
            .iter()
            .map(|record| ProcessInfo {
                process_pid: record.pid,
                process_exe: record.exe.clone(),
                process_cmdline: record.cmdline.clone(),
            })
            .collect::<HashSet<ProcessInfo>>()
            .into_iter()
            .collect();

        processes_infos.sort();
        metrics_names.sort();

        let processes: Vec<ProcessRecord> = processes_infos
            .into_iter()
            .map(|process| {
                let process_metrics: Vec<ProcessMetrics> = metrics_names
                    .clone()
                    .into_iter()
                    .map(|metric_name| {
                        let metric_values = records
                            .iter()
                            .filter(|record| {
                                record.pid == process.process_pid && record.metric == metric_name
                            })
                            .map(|record| record.value)
                            .collect::<Vec<f64>>();

                        ProcessMetrics {
                            metric_name,
                            metric_values,
                        }
                    })
                    .collect();
                ProcessRecord {
                    process,
                    metrics: process_metrics,
                }
            })
            .collect();
        ApiResponseBuilder(ApiResponse {
            project_name: project_name.to_owned(),
            processes,
        })
    }

    pub fn build(self) -> ApiResponse {
        self.0
    }
}

/* #[debug_handler]
pub async fn get_run(
    Extension(db_pool): Extension<PgPool>,
    Extension(project_name): Extension<String>,
    Path(run_id): Path<Uuid>,
) -> Json<Value> {
    let run_rows = select_metrics_from_dimension(db_pool.acquire().await.unwrap(), "runs", run_id)
        .await
        .unwrap();

    let response =
        serde_json::json!(ApiResponseBuilder::new(&run_rows, &project_name).build());
    Json(response)
} */

pub async fn get_run(
    Extension(db_pool): Extension<PgPool>,
    Extension(project_name): Extension<String>,
    Path(run_id): Path<Uuid>,
) -> Json<Value> {
    let run_rows = select_metrics_from_dimension(db_pool.acquire().await.unwrap(), "run", run_id)
        .await
        .unwrap();
    let response =
        serde_json::json!(ApiResponseBuilder::new(&run_rows, &project_name).build());
    Json(response)
}

pub fn app() -> Router {
    let project_root_path = std::env::current_dir().unwrap().join("..");
    let config = Config::check_configuration(&project_root_path)
        .expect("Configuration fields should be parsable.");

    // let database_url = &config.database_url;
    let project_name = &config.project_name;

    // println!("{}", database_url);
    println!("{}", project_name);

    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/runs/:run_id", get(get_run))
}
