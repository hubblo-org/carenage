use crate::boagent::Config;
use crate::ci::GitlabVariables;
use crate::database::{
    format_hardware_data, get_db_connection_pool, get_project_id, insert_device_metadata,
    insert_dimension_table_metadata, to_datetime_local
};
use crate::timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::error::ErrorKind;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::Row;
use uuid::Uuid;
use std::process;

pub trait Metadata {
    fn set_name(&self) -> String;
    fn set_start_date(&self, start_timestamp: Timestamp) -> Timestamp;
    fn serialize(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
    ) -> Value;
    async fn insert(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
    ) -> Result<InsertAttempt, Box<dyn std::error::Error>>;
    async fn get_id(
        &self,
        insert_attempt: InsertAttempt,
        project_name: Option<String>,
    ) -> Result<uuid::Uuid, Box<dyn std::error::Error>>;
}

pub enum CarenageRow {
    Project,
    Workflow,
    Pipeline,
    Job,
    Run,
    Task,
    Device,
}

pub enum InsertAttempt {
    Success(Vec<PgRow>),
    Pending(Result<Vec<PgRow>, sqlx::Error>),
}

impl CarenageRow {
    pub fn table_name(&self) -> &str {
        match self {
            CarenageRow::Project => "projects",
            CarenageRow::Workflow => "workflows",
            CarenageRow::Pipeline => "pipelines",
            CarenageRow::Job => "jobs",
            CarenageRow::Run => "runs",
            CarenageRow::Task => "tasks",
            CarenageRow::Device => "devices",
        }
    }
}

impl Metadata for CarenageRow {
    fn set_name(&self) -> String {
        let project_root_path = std::env::current_dir().unwrap().join("..");
        let config = Config::check_configuration(&project_root_path)
            .expect("Configuration fields should be parsable.");
        let gitlab_vars = GitlabVariables::parse_env_variables()
            .expect("Gitlab variables should be available to parse");
        let row_name: String = match self {
            CarenageRow::Project => gitlab_vars.project_path.to_string(),
            CarenageRow::Workflow => format!("workflow_{}", gitlab_vars.project_path),
            CarenageRow::Pipeline => gitlab_vars.pipeline_name,
            CarenageRow::Job => gitlab_vars.job_name,
            CarenageRow::Run => format!("run_{}", gitlab_vars.job_name),
            CarenageRow::Task => gitlab_vars.job_stage,
            CarenageRow::Device => config.device_name,
        };
        row_name
    }

    fn set_start_date(&self, start_timestamp: Timestamp) -> Timestamp {
        let gitlab_vars = GitlabVariables::parse_env_variables()
            .expect("Gitlab variables should be available to parse");
        let start_date: Option<Timestamp> = match self {
            CarenageRow::Project => Some(start_timestamp),
            CarenageRow::Workflow => Some(gitlab_vars.pipeline_created_at),
            CarenageRow::Pipeline => Some(start_timestamp),
            CarenageRow::Job => Some(gitlab_vars.job_started_at),
            CarenageRow::Run => Some(start_timestamp),
            CarenageRow::Task => Some(start_timestamp),
            CarenageRow::Device => None,
        };

        start_date.expect("Row should receive a parsable timestamp.")
    }

    fn serialize(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
    ) -> Value {
        let project_root_path = std::env::current_dir().unwrap().join("..");
        let config = Config::check_configuration(&project_root_path)
            .expect("Configuration fields should be parsable.");
        match self {
            CarenageRow::Device => format_hardware_data(
                deserialized_boagent_response.expect("Boagent response should be parsable."),
                config.device_name,
                config.location,
                config.lifetime,
            )
            .expect("Formatting of device data should succeed."),
            _ => {
                let name = self.set_name();
                let start_date = self.set_start_date(start_timestamp);
                json!({
                     "name": name,
                     "start_date": start_date.to_string()
                })
            }
        }
    }
    async fn insert(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
    ) -> Result<InsertAttempt, Box<dyn std::error::Error>> {
        let project_root_path = std::env::current_dir().unwrap().join("..");
        let config = Config::check_configuration(&project_root_path)?;
        let db_pool = get_db_connection_pool(config.database_url).await?;
        let rows: InsertAttempt = match self {
            CarenageRow::Project => InsertAttempt::Pending(
                insert_dimension_table_metadata(
                    db_pool.acquire().await?,
                    self.table_name(),
                    self.serialize(start_timestamp, None),
                )
                .await,
            ),
            CarenageRow::Workflow
            | CarenageRow::Pipeline
            | CarenageRow::Job
            | CarenageRow::Run
            | CarenageRow::Task => InsertAttempt::Success(
                insert_dimension_table_metadata(
                    db_pool.acquire().await?,
                    self.table_name(),
                    self.serialize(start_timestamp, None),
                )
                .await?,
            ),
            CarenageRow::Device => InsertAttempt::Success(
                insert_device_metadata(
                    db_pool.acquire().await?,
                    self.serialize(start_timestamp, deserialized_boagent_response),
                )
                .await?,
            ),
        };
        Ok(rows)
    }

    async fn get_id(
        &self,
        insert_attempt: InsertAttempt,
        row_name: Option<String>,
    ) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
        let id: uuid::Uuid = match insert_attempt {
            InsertAttempt::Pending(Ok(rows)) => {
                println!("Inserted {} metadata into database.", self.table_name());
                rows[0].get("id")
            }
            InsertAttempt::Pending(Err(err)) => match err
                .as_database_error()
                .expect("It should be a DatabaseError")
                .kind()
            {
                ErrorKind::UniqueViolation => {
                    println!(
                        "Metadata already present in database, not a project initialization: {}",
                        err
                    );
                    let project_root_path = std::env::current_dir().unwrap().join("..");
                    let config = Config::check_configuration(&project_root_path)?;
                    let db_pool = get_db_connection_pool(config.database_url).await?;
                    get_project_id(db_pool.acquire().await?, row_name.unwrap()).await?
                }
                _ => {
                    eprintln!("Error while processing metadata insertion: {}", err);
                    process::exit(0x0100)
                }
            },
            InsertAttempt::Success(rows) => {
                println!("Inserted {} metadata into database.", self.table_name());
                rows[0].get("id")
            }
        };
        Ok(id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Process {
    pub pid: i32,
    pub exe: String,
    pub cmdline: String,
    pub state: String,
    pub start_date: String,
}

impl Process {
    pub async fn insert(&self, db_connection: PoolConnection<Postgres>) -> Result<PgRow, Box<dyn std::error::Error>> {
        let start_timestamptz = to_datetime_local(&self.start_date);

        let insert_query = "INSERT INTO processes (pid, exe, cmdline, state, start_date) VALUES ($1, $2, $3, $4, $5) RETURNING id";

        let process_row = sqlx::query(insert_query)
            .bind(self.pid)
            .bind(&self.exe)
            .bind(&self.cmdline)
            .bind(&self.state)
            .bind(start_timestamptz)
            .fetch_one(&mut db_connection.detach())
            .await?;

        println!("Inserted process metadata into database.");

        Ok(process_row)
    }
    pub fn get_id(process_row: PgRow) -> Uuid {
        let process_id: Uuid = process_row.get("id");
        process_id
    }
}
