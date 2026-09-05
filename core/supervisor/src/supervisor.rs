use crate::specialist::Specialist;
use crate::delegation::DelegationRequest;
use crate::error::{SupervisorError, SupervisorResult};
use std::collections::HashMap;

pub struct SupervisorAgent {
    specialists: Vec<Specialist>,
    task_queue: Vec<DelegationRequest>,
    completed_tasks: HashMap<String, DelegationRequest>,
}

impl SupervisorAgent {
    pub fn new() -> Self {
        Self {
            specialists: Vec::new(),
            task_queue: Vec::new(),
            completed_tasks: HashMap::new(),
        }
    }

    pub fn add_specialist(&mut self, specialist: Specialist) {
        self.specialists.push(specialist);
    }

    pub fn delegate(&mut self, request: DelegationRequest) -> SupervisorResult<String> {
        let specialist = self.find_suitable_specialist(&request)?;

        let mut req = request.clone();
        req.mark_assigned();
        let task_id = req.id.clone();

        self.task_queue.push(req);

        Ok(format!(
            "Task {} delegated to {}",
            task_id, specialist.name
        ))
    }

    fn find_suitable_specialist(
        &self,
        request: &DelegationRequest,
    ) -> SupervisorResult<Specialist> {
        self.specialists
            .iter()
            .find(|s| {
                s.active
                    && request
                        .required_capabilities
                        .iter()
                        .any(|cap| s.capabilities.contains(cap))
            })
            .cloned()
            .ok_or(SupervisorError::NoSpecialistFound)
    }

    pub fn list_specialists(&self) -> Vec<Specialist> {
        self.specialists.clone()
    }

    pub fn get_pending_tasks(&self) -> Vec<DelegationRequest> {
        self.task_queue.clone()
    }
}

impl Default for SupervisorAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specialist::SpecialistType;

    #[test]
    fn test_supervisor_creation() {
        let supervisor = SupervisorAgent::new();
        assert_eq!(supervisor.specialists.len(), 0);
    }

    #[test]
    fn test_add_specialist() {
        let mut supervisor = SupervisorAgent::new();
        let specialist = Specialist::new(SpecialistType::Coder, "Coder".to_string());

        supervisor.add_specialist(specialist);
        assert_eq!(supervisor.specialists.len(), 1);
    }

    #[test]
    fn test_delegation() {
        let mut supervisor = SupervisorAgent::new();
        supervisor.add_specialist(Specialist::new(
            SpecialistType::Coder,
            "Coder".to_string(),
        ));

        let req = DelegationRequest::new(
            "fix code".to_string(),
            vec!["code_writing".to_string()],
        );
        let result = supervisor.delegate(req);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_specialist_found() {
        let mut supervisor = SupervisorAgent::new();
        let req = DelegationRequest::new(
            "analyze".to_string(),
            vec!["impossible_capability".to_string()],
        );
        let result = supervisor.delegate(req);
        assert!(result.is_err());
    }
}
