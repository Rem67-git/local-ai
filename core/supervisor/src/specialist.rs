use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialistType {
    Planner,
    Coder,
    Tester,
    Analyst,
    Reviewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specialist {
    pub id: String,
    pub spec_type: SpecialistType,
    pub name: String,
    pub capabilities: Vec<String>,
    pub available_tools: Vec<String>,
    pub active: bool,
}

impl Specialist {
    pub fn new(spec_type: SpecialistType, name: String) -> Self {
        let capabilities = match spec_type {
            SpecialistType::Planner => vec![
                "goal_decomposition".to_string(),
                "strategy_selection".to_string(),
            ],
            SpecialistType::Coder => vec![
                "code_reading".to_string(),
                "code_writing".to_string(),
                "error_detection".to_string(),
            ],
            SpecialistType::Tester => vec![
                "test_execution".to_string(),
                "test_writing".to_string(),
            ],
            SpecialistType::Analyst => vec![
                "data_analysis".to_string(),
                "pattern_detection".to_string(),
            ],
            SpecialistType::Reviewer => vec![
                "code_review".to_string(),
                "quality_assessment".to_string(),
            ],
        };

        Self {
            id: Uuid::new_v4().to_string(),
            spec_type,
            name,
            capabilities,
            available_tools: vec![],
            active: true,
        }
    }

    pub fn can_handle(&self, task: &str) -> bool {
        self.active && self.capabilities.iter().any(|c| task.contains(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specialist_creation() {
        let specialist = Specialist::new(SpecialistType::Coder, "Code Master".to_string());
        assert_eq!(specialist.spec_type, SpecialistType::Coder);
        assert!(specialist.active);
    }

    #[test]
    fn test_can_handle() {
        let coder = Specialist::new(SpecialistType::Coder, "Coder".to_string());
        assert!(coder.can_handle("code_writing"));
        assert!(!coder.can_handle("goal_decomposition"));
    }
}
