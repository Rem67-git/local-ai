use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub status: GoalStatus,
    pub dependencies: Vec<String>,
    pub estimated_complexity: u8,
}

impl Goal {
    pub fn new(id: String, description: String, dependencies: Vec<String>) -> Self {
        Self {
            id,
            description,
            status: GoalStatus::Pending,
            dependencies,
            estimated_complexity: 5,
        }
    }

    pub fn set_complexity(&mut self, complexity: u8) {
        self.estimated_complexity = complexity.min(10);
    }
}
