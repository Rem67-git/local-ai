pub mod dag;
pub mod goal;
pub mod plan;
pub mod replanner;
pub mod strategy;

#[cfg(test)]
mod tests;

pub use dag::DAG;
pub use goal::{Goal, GoalStatus};
pub use plan::Plan;
pub use strategy::Strategy;

use inference::LLMRuntime;
use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("Planning failed: {0}")]
    PlanningFailed(String),

    #[error("DAG validation failed: {0}")]
    DAGValidation(String),

    #[error("Cycle detected in DAG")]
    CycleDetected,
}

pub type PlannerResult<T> = Result<T, PlannerError>;

pub struct Planner;

impl Planner {
    pub async fn plan_mission(
        goal: &str,
        _context: &str,
        _llm: &dyn LLMRuntime,
    ) -> PlannerResult<Plan> {
        info!("Planning mission: {}", goal);

        let mut sub_goals = vec![];
        sub_goals.push(Goal::new(
            uuid::Uuid::new_v4().to_string(),
            format!("Step 1: {}", goal),
            vec![],
        ));

        let dag = DAG::from_goals(sub_goals)?;
        dag.validate_acyclic()?;

        let execution_order = dag.topological_sort();
        info!("Plan created with {} sub-goals", execution_order.len());

        Ok(Plan {
            mission_goal: goal.to_string(),
            sub_goals: dag,
            strategy: Strategy::Sequential,
            execution_order,
        })
    }

    pub async fn replan(
        _plan: &Plan,
        _failed_goal: &str,
        _context: &str,
        _llm: &dyn LLMRuntime,
    ) -> PlannerResult<Plan> {
        Err(PlannerError::PlanningFailed(
            "Replanning not yet implemented".to_string(),
        ))
    }
}
