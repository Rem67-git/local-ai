use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    ActionStarted { action: String },
    ActionCompleted { action: String, result: String },
    ActionFailed { action: String, error: String },
    MissionStarted { mission_id: String },
    MissionCheckpoint { mission_id: String, progress: String },
    MissionComplete { mission_id: String },
    MissionError { mission_id: String, error: String },
    BudgetWarning { mission_id: String, remaining: String },
    BudgetExceeded { mission_id: String },
    LoopDetected { mission_id: String, action: String },
    HumanApprovalRequired { mission_id: String, action: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
        }
    }

    pub fn summary(&self) -> String {
        match &self.event_type {
            EventType::ActionStarted { action } => format!("Action started: {}", action),
            EventType::ActionCompleted { action, result } => {
                format!("Action completed: {} → {}", action, result)
            }
            EventType::ActionFailed { action, error } => {
                format!("Action failed: {} - {}", action, error)
            }
            EventType::MissionStarted { mission_id } => format!("Mission started: {}", mission_id),
            EventType::MissionCheckpoint { mission_id, progress } => {
                format!("Mission checkpoint: {} - {}", mission_id, progress)
            }
            EventType::MissionComplete { mission_id } => format!("Mission complete: {}", mission_id),
            EventType::MissionError { mission_id, error } => {
                format!("Mission error: {} - {}", mission_id, error)
            }
            EventType::BudgetWarning { mission_id, remaining } => {
                format!("Budget warning: {} - {}", mission_id, remaining)
            }
            EventType::BudgetExceeded { mission_id } => format!("Budget exceeded: {}", mission_id),
            EventType::LoopDetected { mission_id, action } => {
                format!("Loop detected: {} - {}", mission_id, action)
            }
            EventType::HumanApprovalRequired { mission_id, action } => {
                format!("Approval required: {} - {}", mission_id, action)
            }
        }
    }
}
