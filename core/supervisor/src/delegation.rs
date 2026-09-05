use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRequest {
    pub id: String,
    pub task: String,
    pub required_capabilities: Vec<String>,
    pub priority: u8,
    pub status: DelegationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelegationStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
}

impl DelegationRequest {
    pub fn new(task: String, capabilities: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task,
            required_capabilities: capabilities,
            priority: 5,
            status: DelegationStatus::Pending,
        }
    }

    pub fn mark_assigned(&mut self) {
        self.status = DelegationStatus::Assigned;
    }

    pub fn mark_started(&mut self) {
        self.status = DelegationStatus::InProgress;
    }

    pub fn mark_completed(&mut self) {
        self.status = DelegationStatus::Completed;
    }

    pub fn mark_failed(&mut self) {
        self.status = DelegationStatus::Failed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation_creation() {
        let req = DelegationRequest::new(
            "analyze code".to_string(),
            vec!["code_reading".to_string()],
        );
        assert_eq!(req.status, DelegationStatus::Pending);
    }

    #[test]
    fn test_status_transitions() {
        let mut req = DelegationRequest::new("task".to_string(), vec![]);
        req.mark_assigned();
        assert_eq!(req.status, DelegationStatus::Assigned);
        req.mark_started();
        assert_eq!(req.status, DelegationStatus::InProgress);
        req.mark_completed();
        assert_eq!(req.status, DelegationStatus::Completed);
    }
}
