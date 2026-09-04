use crate::{DAG, Strategy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub mission_goal: String,
    pub sub_goals: DAG,
    pub strategy: Strategy,
    pub execution_order: Vec<String>,
}
