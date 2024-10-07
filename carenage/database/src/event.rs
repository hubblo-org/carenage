use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
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

pub struct Event {
    pub project_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub device_id: Uuid,
    pub event_type: EventType,
}
