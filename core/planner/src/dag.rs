use crate::goal::Goal;
use crate::PlannerResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        // Kahn's algorithm for topological sort
        let mut in_degree = HashMap::new();
        for goal_id in self.goals.keys() {
            in_degree.insert(goal_id.clone(), 0);
        }

        for (_parent, child) in &self.edges {
            *in_degree.get_mut(child).unwrap_or(&mut 0) += 1;
        }

        let mut queue: VecDeque<String> = self.goals
            .keys()
            .filter(|id| in_degree[*id] == 0)
            .cloned()
            .collect();

        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            for (_parent, child) in &self.edges {
                if _parent == &node {
                    in_degree.insert(child.clone(), in_degree[child] - 1);
                    if in_degree[child] == 0 {
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        result
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
