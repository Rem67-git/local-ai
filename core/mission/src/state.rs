use crate::task::Task;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MissionStatus {
    Pending,
    InProgress,
    Complete,
    Error,
}

impl std::fmt::Display for MissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionStatus::Pending => write!(f, "pending"),
            MissionStatus::InProgress => write!(f, "in_progress"),
            MissionStatus::Complete => write!(f, "complete"),
            MissionStatus::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionState {
    pub id: String,
    pub goal: String,
    pub status: MissionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tasks: Vec<Task>,
    pub observations: Vec<ObservationRecord>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationRecord {
    pub timestamp: DateTime<Utc>,
    pub tool: String,
    pub success: bool,
    pub output: String,
}

impl ObservationRecord {
    pub fn summary(&self) -> String {
        if self.success {
            format!("✅ {}: {}", self.tool, self.output)
        } else {
            format!("❌ {}: {}", self.tool, self.output)
        }
    }
}

impl MissionState {
    pub fn new(goal: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            goal,
            status: MissionStatus::Pending,
            created_at: now,
            updated_at: now,
            tasks: Vec::new(),
            observations: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_observation_record(&mut self, tool: String, success: bool, output: String) {
        self.updated_at = Utc::now();
        let record = ObservationRecord {
            timestamp: self.updated_at,
            tool,
            success,
            output,
        };
        self.observations.push(record);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn is_complete(&self) -> bool {
        self.status == MissionStatus::Complete
    }

    pub fn duration(&self) -> chrono::Duration {
        self.updated_at - self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_creation() {
        let mission = MissionState::new("Test mission".to_string());
        assert_eq!(mission.goal, "Test mission");
        assert_eq!(mission.status, MissionStatus::Pending);
        assert!(!mission.is_complete());
    }

    #[test]
    fn test_mission_completion() {
        let mut mission = MissionState::new("Test mission".to_string());
        mission.status = MissionStatus::Complete;
        assert!(mission.is_complete());
    }
}
