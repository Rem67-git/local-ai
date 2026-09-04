/// End-to-end integration test: Phase 1 (LLM) + Phase 2 (Agent) + Phase 4 (Tools)
/// This demonstrates the full mission execution pipeline
#[cfg(test)]
mod e2e_tests {
    // Once Phase 1 builds, add:
    // use inference::{LLMRuntime, OllamaRuntime};
    // use agent::{Agent, AgentConfig};
    // use mission::MissionState;

    #[test]
    #[ignore] // Requires build to complete
    fn test_full_mission_execution() {
        // let llm = OllamaRuntime::new("http://localhost:11434".to_string(), "mistral".to_string());
        // let config = AgentConfig::default();
        // let mut agent = Agent::new(Arc::new(llm), config);
        // let mut mission = MissionState::new("List files in current directory".to_string());
        //
        // tokio::runtime::Runtime::new().unwrap().block_on(async {
        //     let result = agent.run_mission(&mut mission).await;
        //     assert!(result.is_ok());
        //     assert!(!mission.observations.is_empty());
        // });
    }

    #[test]
    #[ignore] // Requires build to complete
    fn test_mission_with_budget_enforcement() {
        // Test that budget limits are enforced
        // let config = AgentConfig {
        //     token_budget: 100, // Very small budget
        //     action_budget: 2,
        //     time_budget_seconds: 10,
        //     max_iterations: 10,
        // };
        //
        // let mut agent = Agent::new(Arc::new(llm), config);
        // let result = agent.run_mission(&mut mission).await;
        // assert!(matches!(result, Err(AgentError::BudgetExceeded(_))));
    }

    #[test]
    #[ignore] // Requires build to complete
    fn test_loop_detection() {
        // Test that repeated actions are detected
        // Agent should detect pattern and stop
    }

    #[test]
    fn test_mission_state_lifecycle() {
        use mission::{MissionState, MissionStatus};

        let mut mission = MissionState::new("Test mission".to_string());
        assert_eq!(mission.status, MissionStatus::Pending);

        mission.status = MissionStatus::InProgress;
        assert_eq!(mission.status, MissionStatus::InProgress);

        mission.add_observation_record(
            "test_tool".to_string(),
            true,
            "Success output".to_string(),
        );
        assert_eq!(mission.observations.len(), 1);

        mission.status = MissionStatus::Complete;
        assert!(mission.is_complete());
    }

    #[test]
    fn test_event_bus_broadcast() {
        use events::{Event, EventType, EventBus};

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let bus = EventBus::new(10);
            let mut rx = bus.subscribe();

            let event = Event::new(EventType::ActionStarted {
                action: "file_read".to_string(),
            });

            bus.emit(event.clone()).await;
            let received = rx.recv().await;
            assert!(received.is_ok());
        });
    }

    #[test]
    fn test_structured_action_parsing() {
        use agent::StructuredAction;

        let json = r#"{
            "action_type": "tool_call",
            "tool": "file_read",
            "args": {"path": "/tmp/test.txt"},
            "reasoning": "Reading test file"
        }"#;

        let action = StructuredAction::from_json(json);
        assert!(action.is_ok());

        let action = action.unwrap();
        assert_eq!(action.action_type, "tool_call");
        assert_eq!(action.tool, "file_read");
        assert!(action.args.contains_key("path"));
    }

    #[test]
    fn test_budget_enforcement() {
        use agent::Budget;

        let mut budget = Budget::new(1000, 10, 60);
        assert!(budget.consume_tokens(500).is_ok());
        assert!(budget.consume_tokens(500).is_ok());
        assert!(budget.consume_tokens(1).is_err()); // Exceeds limit

        let mut budget = Budget::new(1000, 3, 60);
        assert!(budget.consume_action().is_ok());
        assert!(budget.consume_action().is_ok());
        assert!(budget.consume_action().is_ok());
        assert!(budget.consume_action().is_err()); // Exceeds limit
    }

    #[test]
    fn test_loop_detection_basic() {
        use agent::LoopDetector;

        let mut detector = LoopDetector::new(10);
        detector.record_action("action_a".to_string());
        detector.record_action("action_b".to_string());
        detector.record_action("action_a".to_string());
        detector.record_action("action_a".to_string());
        detector.record_action("action_a".to_string());

        assert!(detector.is_looping());
    }
}
