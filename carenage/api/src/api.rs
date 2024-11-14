use axum::extract::Request;
use axum::Extension;
use axum::{debug_handler, extract::Path, http::Uri, response::Json, routing::get, Router};
use chrono::{DateTime, Local};
use database::database::{
    select_metrics_from_dimension, select_project_name_from_dimension, Record,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use uuid::Uuid;
use crate::utils::format_uri_to_dimension;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProcessInfo {
    pub process_pid: i32,
    pub process_exe: String,
    pub process_cmdline: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessMetrics {
    pub metric_name: String,
    pub metric_values: Vec<(DateTime<Local>, f64)>,
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
                            .map(|record| (record.timestamp, record.value))
                            .collect::<Vec<(DateTime<Local>, f64)>>();

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

#[debug_handler]
pub async fn get_dimension(
    Extension(db_pool): Extension<PgPool>,
    Path(dimension_id): Path<Uuid>,
    request: Request,
) -> Json<ApiResponse> {

    let uri = request.uri();
    let dimension = format_uri_to_dimension(uri);

    let project_name = select_project_name_from_dimension(
        db_pool.acquire().await.unwrap(),
        &dimension,
        dimension_id,
    )
    .await
    .unwrap()
    .get::<&str, &str>("name")
    .to_owned();

    let rows =
        select_metrics_from_dimension(db_pool.acquire().await.unwrap(), &dimension, dimension_id)
            .await
            .unwrap();
    let response = ApiResponseBuilder::new(&rows, &project_name).build();
    Json(response)
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to the Carenage API!" }))
        .route("/runs/:run_id", get(get_dimension))
        .route("/projects/:project_id", get(get_dimension))
        .route("/workflows/:workflow_id", get(get_dimension))
        .route("/pipelines/:pipeline_id", get(get_dimension))
        .route("/jobs/:job_id", get(get_dimension))
        .route("/tasks/:task_id", get(get_dimension))
}
