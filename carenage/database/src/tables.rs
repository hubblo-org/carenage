use crate::boagent::Config;
use crate::ci::GitlabVariables;
use crate::database::{
    format_hardware_data, get_db_connection_pool, get_project_id, insert_device_metadata,
    insert_dimension_table_metadata
};
use crate::timestamp::Timestamp;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use sqlx::error::ErrorKind;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::Row;
use std::process;
use uuid::Uuid;

pub trait Metadata {
    fn set_name(&self, config: &Config) -> String;
    fn set_start_date(&self, start_timestamp: Timestamp) -> Timestamp;
    fn serialize(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
        config: &Config,
    ) -> Value;
    async fn insert(
        &self,
        start_timestamp: Timestamp,
        deserialized_boagent_response: Option<Value>,
        config: &Config,
    ) -> Result<InsertAttempt, Box<dyn std::error::Error>>;
    async fn get_id(
        &self,
        insert_attempt: InsertAttempt,
        project_name: Option<&String>,
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
    Success(PgRow),
    Pending(Result<PgRow, sqlx::Error>),
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
    fn set_name(&self, config: &Config) -> String {
        let gitlab_vars = GitlabVariables::parse_env_variables()
            .expect("Gitlab variables should be available to parse");
        let row_name: String = match self {
            CarenageRow::Project => gitlab_vars.project_path.to_string(),
            CarenageRow::Workflow => format!("workflow_{}", gitlab_vars.project_path),
            CarenageRow::Pipeline => gitlab_vars.pipeline_name,
            CarenageRow::Job => gitlab_vars.job_name,
            CarenageRow::Run => format!("run_{}", gitlab_vars.job_name),
            CarenageRow::Task => gitlab_vars.job_stage,
            CarenageRow::Device => config.device_name.clone(),
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
        config: &Config,
    ) -> Value {
        match self {
            CarenageRow::Device => format_hardware_data(
                deserialized_boagent_response.expect("Boagent response should be parsable."),
                &config.device_name,
                &config.location,
                config.lifetime,
            )
            .expect("Formatting of device data should succeed."),
            _ => {
                let name = self.set_name(config);
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
        config: &Config,
    ) -> Result<InsertAttempt, Box<dyn std::error::Error>> {
        let db_pool = get_db_connection_pool(&config.database_url).await?;
        let rows: InsertAttempt = match self {
            CarenageRow::Project => InsertAttempt::Pending(
                insert_dimension_table_metadata(
                    db_pool.acquire().await?,
                    self.table_name(),
                    self.serialize(start_timestamp, None, config),
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
                    self.serialize(start_timestamp, None, config),
                )
                .await?,
            ),
            CarenageRow::Device => InsertAttempt::Success(
                insert_device_metadata(
                    db_pool.acquire().await?,
                    self.serialize(start_timestamp, deserialized_boagent_response, config),
                )
                .await?,
            ),
        };
        Ok(rows)
    }

    async fn get_id(
        &self,
        insert_attempt: InsertAttempt,
        row_name: Option<&String>,
    ) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
        let id: uuid::Uuid = match insert_attempt {
            InsertAttempt::Pending(Ok(row)) => {
                info!("Inserted {} metadata into database.", self.table_name());
                row.get("id")
            }
            InsertAttempt::Pending(Err(err)) => match err
                .as_database_error()
                .expect("It should be a DatabaseError.")
                .kind()
            {
                ErrorKind::UniqueViolation => {
                    info!(
                        "Metadata already present in database, not a project initialization: {}.",
                        err
                    );
                    let project_root_path = std::env::current_dir().unwrap().join("..");
                    let config = Config::check_configuration(&project_root_path)?;
                    let db_pool = get_db_connection_pool(&config.database_url).await?;
                    get_project_id(db_pool.acquire().await?, row_name.unwrap()).await?
                }
                _ => {
                    error!("Error while processing metadata insertion: {}", err);
                    process::exit(0x0100)
                }
            },
            InsertAttempt::Success(row) => {
                info!("Inserted {} metadata into database.", self.table_name());
                row.get("id")
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
}

pub struct ProcessBuilder(Process);

impl ProcessBuilder {
    pub fn new(pid: i32, exe: &str, cmdline: &str, state: &str) -> Self {
        ProcessBuilder(Process {
            pid,
            exe: exe.to_owned(),
            cmdline: cmdline.to_owned(),
            state: state.to_owned(),
        })
    }
    pub fn build(self) -> Process {
        self.0
    }
}

impl Process {
    pub async fn insert(
        &self,
        db_connection: PoolConnection<Postgres>,
    ) -> Result<PgRow, Box<dyn std::error::Error>> {
        let insert_query =
            "INSERT INTO processes (pid, exe, cmdline, state) VALUES ($1, $2, $3, $4) RETURNING id";

        let process_row = sqlx::query(insert_query)
            .bind(self.pid)
            .bind(&self.exe)
            .bind(&self.cmdline)
            .bind(&self.state)
            .fetch_one(&mut db_connection.detach())
            .await?;

        info!("Inserted process metadata into database.");

        Ok(process_row)
    }
    pub fn get_id(process_row: PgRow) -> Uuid {
        let process_id: Uuid = process_row.get("id");
        process_id
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
// https://serde.rs/enum-representations.html#untagged
#[serde(untagged)]
pub enum CharacteristicValue {
    StringValue(String),
    NumericValue(Number),
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub location: String,
    pub lifetime: Number,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    name: String,
    model: String,
    manufacturer: String,
    characteristics: Vec<ComponentCharacteristic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentCharacteristic {
    pub name: String,
    pub value: CharacteristicValue,
}

pub struct DeviceBuilder(Device);
pub struct ComponentBuilder(Component);
pub struct ComponentCharacteristicBuilder(ComponentCharacteristic);

impl DeviceBuilder {
    pub fn new(name: &str, location: &str, lifetime: i16) -> Self {
        DeviceBuilder(Device {
            name: name.to_owned(),
            location: location.to_owned(),
            lifetime: lifetime.into(),
        })
    }
    pub fn build(self) -> Device {
        self.0
    }
}

impl ComponentBuilder {
    pub fn new(
        name: &str,
        model: &str,
        manufacturer: &str,
        characteristics: Vec<ComponentCharacteristic>,
    ) -> Self {
        ComponentBuilder(Component {
            name: name.to_owned(),
            model: model.to_owned(),
            manufacturer: manufacturer.to_owned(),
            characteristics,
        })
    }
    pub fn build(self) -> Component {
        self.0
    }
}

impl ComponentCharacteristicBuilder {
    pub fn new(name: &str, value: CharacteristicValue) -> Self {
        ComponentCharacteristicBuilder(ComponentCharacteristic {
            name: name.to_owned(),
            value,
        })
    }
    pub fn build(self) -> ComponentCharacteristic {
        self.0
    }
}
