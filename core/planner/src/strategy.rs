use crate::Plan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Strategy {
    Sequential,
    Parallel { max_concurrent: u8 },
    Conditional { branches: Vec<(String, Plan)> },
    Branching,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Sequential
    }
}
