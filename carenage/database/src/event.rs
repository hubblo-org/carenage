use crate::timestamp::Timestamp;
use uuid::Uuid;

pub enum EventType {
    Regular,
    Custom,
    Start,
    Stop,
}

pub struct Event {
    pub id: Uuid,
    pub timestamp: Timestamp,
    pub project_id: Uuid,
    pub repository_id: Uuid,
    pub workflow_id: Uuid,
    pub pipeline_id: Uuid,
    pub job_id: Uuid,
    pub run_id: Uuid,
    pub task_id: Uuid,
    pub device_id: Uuid,
    pub event_type: EventType,
    pub user_label: String,
}
