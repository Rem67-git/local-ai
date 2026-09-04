use crate::goal::Goal;
use crate::PlannerResult;
use std::collections::HashMap;

pub struct DAG {
    pub goals: HashMap<String, Goal>,
    pub edges: Vec<(String, String)>,
}

impl DAG {
    pub fn from_goals(goals: Vec<Goal>) -> PlannerResult<Self> {
        let mut map = HashMap::new();
        let mut edges = Vec::new();
        for g in goals {
            for dep in &g.dependencies {
                edges.push((dep.clone(), g.id.clone()));
            }
            map.insert(g.id.clone(), g);
        }
        Ok(Self { goals: map, edges })
    }

    pub fn validate_acyclic(&self) -> PlannerResult<()> {
        Ok(())
    }

    pub fn topological_sort(&self) -> Vec<String> {
        self.goals.keys().cloned().collect()
    }

    pub fn get_blocking_goals(&self, _goal_id: &str) -> Vec<String> {
        Vec::new()
    }

    pub fn next_executable(&self) -> Vec<String> {
        self.goals
            .values()
            .filter(|g| g.dependencies.is_empty())
            .map(|g| g.id.clone())
            .collect()
    }
}
