use crate::boagent::Config;
use crate::ci::GitlabVariables;
use crate::database::{get_db_connection_pool, get_project_id, insert_dimension_table_metadata};
use crate::timestamp::Timestamp;
use serde_json::{json, Value};
use sqlx::error::ErrorKind;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::process;

pub trait Create {
    fn set_name(&self) -> String;
    fn set_start_date(&self, start_timestamp: Timestamp) -> Timestamp;
    fn serialize(&self, start_timestamp: Timestamp) -> Value;
}

pub trait Insert {
    async fn insert(
        &self,
        start_timestamp: Timestamp,
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
        }
    }
}

impl Create for CarenageRow {
    fn set_name(&self) -> String {
        let gitlab_vars = GitlabVariables::parse_env_variables()
            .expect("Gitlab variables should be available to parse");
        let row_name: String = match self {
            CarenageRow::Project => gitlab_vars.project_path.to_string(),
            CarenageRow::Workflow => format!("workflow_{}", gitlab_vars.project_path),
            CarenageRow::Pipeline => gitlab_vars.pipeline_name,
            CarenageRow::Job => gitlab_vars.job_name,
            CarenageRow::Run => format!("run_{}", gitlab_vars.job_name),
            CarenageRow::Task => gitlab_vars.job_stage,
        };
        row_name
    }

    fn set_start_date(&self, start_timestamp: Timestamp) -> Timestamp {
        let gitlab_vars = GitlabVariables::parse_env_variables()
            .expect("Gitlab variables should be available to parse");
        let start_date: Timestamp = match self {
            CarenageRow::Project => start_timestamp,
            CarenageRow::Workflow => gitlab_vars.pipeline_created_at,
            CarenageRow::Pipeline => start_timestamp,
            CarenageRow::Job => gitlab_vars.job_started_at,
            CarenageRow::Run => start_timestamp,
            CarenageRow::Task => start_timestamp,
        };

        start_date
    }

    fn serialize(&self, start_timestamp: Timestamp) -> Value {
        let name = self.set_name();
        let start_date = self.set_start_date(start_timestamp);
        json!({
             "name": name,
             "start_date": start_date.to_string()
        })
    }
}

impl Insert for CarenageRow {
    async fn insert(
        &self,
        start_timestamp: Timestamp,
    ) -> Result<InsertAttempt, Box<dyn std::error::Error>> {
        let project_root_path = std::env::current_dir().unwrap().join("..");
        let config = Config::check_configuration(&project_root_path)?;
        let db_pool = get_db_connection_pool(config.database_url).await?;
        let rows: InsertAttempt = match self {
            CarenageRow::Project => InsertAttempt::Pending(
                insert_dimension_table_metadata(
                    db_pool.acquire().await?,
                    self.table_name(),
                    self.serialize(start_timestamp),
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
                    self.serialize(start_timestamp),
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
