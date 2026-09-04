use crate::actions::StructuredAction;
use crate::budget::{Budget, BudgetTracker};
use crate::executor::Executor;
use crate::loop_detection::LoopDetector;
use crate::observations::ObservationResult;
use inference::LLMRuntime;
use log::{info, warn};
use mission::{MissionState, MissionStatus};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Budget exceeded: {0}")]
    BudgetExceeded(String),

    #[error("Loop detected: {0}")]
    LoopDetected(String),

    #[error("Mission error: {0}")]
    MissionError(String),

    #[error("LLM error: {0}")]
    LLMError(String),
}

pub type AgentResult<T> = Result<T, AgentError>;

pub struct AgentConfig {
    pub max_iterations: usize,
    pub token_budget: usize,
    pub action_budget: usize,
    pub time_budget_seconds: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            token_budget: 10000,
            action_budget: 50,
            time_budget_seconds: 3600,
        }
    }
}

pub struct Agent {
    llm: Arc<dyn LLMRuntime>,
    config: AgentConfig,
    executor: Executor,
    budget_tracker: BudgetTracker,
    loop_detector: LoopDetector,
}

impl Agent {
    pub fn new(llm: Arc<dyn LLMRuntime>, config: AgentConfig) -> Self {
        let budget = Budget::new(
            config.token_budget,
            config.action_budget,
            config.time_budget_seconds,
        );

        Self {
            llm,
            config,
            executor: Executor::new(),
            budget_tracker: BudgetTracker::new(budget),
            loop_detector: LoopDetector::new(10),
        }
    }

    pub async fn run_mission(&mut self, mission: &mut MissionState) -> AgentResult<()> {
        info!("Starting mission: {}", mission.goal);
        mission.status = MissionStatus::InProgress;

        let mut iteration = 0;
        while iteration < self.config.max_iterations && !self.budget_tracker.exceeded() {
            iteration += 1;

            if self.loop_detector.is_looping() {
                warn!("Loop detected in mission {}", mission.id);
                return Err(AgentError::LoopDetected(
                    "Agent is repeating the same action".to_string(),
                ));
            }

            let context = self.build_context(mission);

            let response = self
                .llm
                .infer(&context, Default::default())
                .map_err(|e| AgentError::LLMError(e.to_string()))?;

            let action = StructuredAction::from_json(&response)
                .map_err(|e| AgentError::MissionError(format!("Invalid action JSON: {}", e)))?;

            self.loop_detector
                .record_action(format!("{}:{}", action.action_type, action.tool));

            self.budget_tracker
                .consume_action()
                .map_err(|e| AgentError::BudgetExceeded(e.to_string()))?;

            info!("Executing action {}: {}", iteration, action.tool);

            let observation = self.executor.execute(&action).await.map_err(|e| {
                AgentError::MissionError(format!("Execution failed: {}", e))
            })?;

            info!("{}", observation.summary());
            mission.add_observation_record(
                observation.tool,
                observation.success,
                if observation.success {
                    observation.output
                } else {
                    observation.error.unwrap_or_default()
                },
            );

            if mission.is_complete() {
                mission.status = MissionStatus::Complete;
                info!("Mission completed: {}", mission.goal);
                return Ok(());
            }
        }

        if self.budget_tracker.exceeded() {
            mission.status = MissionStatus::Error;
            return Err(AgentError::BudgetExceeded(
                "Budget exceeded during mission".to_string(),
            ));
        }

        mission.status = MissionStatus::InProgress;
        Ok(())
    }

    fn build_context(&self, mission: &MissionState) -> String {
        format!(
            "Goal: {}\nAvailable tools: {:?}\nBudget remaining: {} tokens, {} actions\nRecent observations: {}",
            mission.goal,
            self.executor.available_tools(),
            self.budget_tracker.budget().remaining_tokens(),
            self.budget_tracker.budget().remaining_actions(),
            mission.observations.iter()
                .rev()
                .take(3)
                .map(|o| o.summary())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.token_budget, 10000);
        assert_eq!(config.action_budget, 50);
    }
}
