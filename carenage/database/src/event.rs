use chrono::Local;
use sqlx::postgres::Postgres;
use sqlx::Row;
use sqlx::{pool::PoolConnection, postgres::PgRow};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::database::Ids;

#[derive(sqlx::Type, Default, Clone, Copy, Debug)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    #[default]
    Regular,
    Custom,
    Start,
    Stop,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            EventType::Regular => {
                write!(f, "regular")
            }
            EventType::Custom => {
                write!(f, "custom")
            }
            EventType::Start => {
                write!(f, "start")
            }
            EventType::Stop => {
                write!(f, "stop")
            }
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub project_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub process_id: Uuid,
    pub device_id: Uuid,
    pub event_type: EventType,
}

pub struct EventBuilder(Event);

impl EventBuilder {
    pub fn new(ids: Ids, event_type: EventType) -> Self {
        EventBuilder(Event {
            project_id: ids.project_id,
            workflow_id: ids.workflow_id,
            pipeline_id: ids.pipeline_id,
            job_id: ids.job_id,
            run_id: ids.run_id,
            task_id: ids.task_id,
            process_id: ids.process_id,
            device_id: ids.device_id,
            event_type,
        })
    }
    pub fn build(self) -> Event {
        self.0
    }
}

impl Event {
    pub async fn insert(
        &self,
        db_connection: PoolConnection<Postgres>,
    ) -> Result<PgRow, Box<dyn std::error::Error>> {
        let formatted_query = "INSERT INTO events (project_id, workflow_id, pipeline_id, job_id, run_id, task_id, process_id, device_id, event_type) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
    RETURNING id";

        let event_row = sqlx::query(formatted_query)
            .bind(self.project_id)
            .bind(self.workflow_id)
            .bind(self.pipeline_id)
            .bind(self.job_id)
            .bind(self.run_id)
            .bind(self.task_id)
            .bind(self.process_id)
            .bind(self.device_id)
            .bind(self.event_type)
            .fetch_one(&mut db_connection.detach())
            .await?;
        Ok(event_row)
    }
    pub fn get_id(event_row: PgRow) -> Uuid {
        let event_id: Uuid = event_row.get("id");
        event_id
    }
}
