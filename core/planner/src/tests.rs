#[cfg(test)]
mod tests {
    use crate::{Goal, GoalStatus, DAG};

    #[test]
    fn test_goal_creation() {
        let goal = Goal {
            id: "g1".to_string(),
            description: "Test goal".to_string(),
            status: GoalStatus::Pending,
            dependencies: vec![],
            estimated_complexity: 5,
        };

        assert_eq!(goal.id, "g1");
        assert_eq!(goal.status, GoalStatus::Pending);
    }

    #[test]
    fn test_goal_status_transitions() {
        let mut goal = Goal {
            id: "g1".to_string(),
            description: "Test".to_string(),
            status: GoalStatus::Pending,
            dependencies: vec![],
            estimated_complexity: 5,
        };

        goal.status = GoalStatus::InProgress;
        assert_eq!(goal.status, GoalStatus::InProgress);

        goal.status = GoalStatus::Completed;
        assert_eq!(goal.status, GoalStatus::Completed);
    }

    #[test]
    fn test_dag_from_goals() {
        let goals = vec![
            Goal {
                id: "g1".to_string(),
                description: "First".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec![],
                estimated_complexity: 3,
            },
            Goal {
                id: "g2".to_string(),
                description: "Second".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec!["g1".to_string()],
                estimated_complexity: 4,
            },
            Goal {
                id: "g3".to_string(),
                description: "Third".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec!["g2".to_string()],
                estimated_complexity: 5,
            },
        ];

        let dag = DAG::from_goals(goals).unwrap();
        assert_eq!(dag.goals.len(), 3);
        assert_eq!(dag.edges.len(), 2);
    }

    #[test]
    fn test_dag_topological_sort() {
        let goals = vec![
            Goal {
                id: "g1".to_string(),
                description: "First".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec![],
                estimated_complexity: 1,
            },
            Goal {
                id: "g2".to_string(),
                description: "Second".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec!["g1".to_string()],
                estimated_complexity: 2,
            },
            Goal {
                id: "g3".to_string(),
                description: "Third".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec!["g1".to_string()],
                estimated_complexity: 2,
            },
        ];

        let dag = DAG::from_goals(goals).unwrap();
        let order = dag.topological_sort();

        assert_eq!(order.len(), 3);
        // g1 should come before g2 and g3
        let g1_idx = order.iter().position(|x| x == "g1").unwrap();
        let g2_idx = order.iter().position(|x| x == "g2").unwrap();
        let g3_idx = order.iter().position(|x| x == "g3").unwrap();
        assert!(g1_idx < g2_idx);
        assert!(g1_idx < g3_idx);
    }

    #[test]
    fn test_dag_next_executable() {
        let goals = vec![
            Goal {
                id: "g1".to_string(),
                description: "First".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec![],
                estimated_complexity: 1,
            },
            Goal {
                id: "g2".to_string(),
                description: "Second".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec!["g1".to_string()],
                estimated_complexity: 2,
            },
        ];

        let dag = DAG::from_goals(goals).unwrap();
        let executable = dag.next_executable();

        assert_eq!(executable.len(), 1);
        assert!(executable.contains(&"g1".to_string()));
        assert!(!executable.contains(&"g2".to_string()));
    }

    #[test]
    fn test_dag_validate_acyclic() {
        let goals = vec![
            Goal {
                id: "g1".to_string(),
                description: "First".to_string(),
                status: GoalStatus::Pending,
                dependencies: vec![],
                estimated_complexity: 1,
            },
        ];

        let dag = DAG::from_goals(goals).unwrap();
        assert!(dag.validate_acyclic().is_ok());
    }
}
