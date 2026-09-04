use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopDetector {
    last_actions: VecDeque<String>,
    max_history: usize,
}

impl LoopDetector {
    pub fn new(max_history: usize) -> Self {
        Self {
            last_actions: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    pub fn record_action(&mut self, action: String) {
        self.last_actions.push_back(action);
        if self.last_actions.len() > self.max_history {
            self.last_actions.pop_front();
        }
    }

    pub fn is_looping(&self) -> bool {
        if self.last_actions.len() < 3 {
            return false;
        }

        let recent: Vec<_> = self.last_actions.iter().rev().take(3).collect();
        recent[0] == recent[1] && recent[1] == recent[2]
    }

    pub fn is_stalled(&self) -> bool {
        if self.last_actions.len() < 5 {
            return false;
        }

        let recent: Vec<_> = self.last_actions.iter().rev().take(5).collect();
        recent.windows(2).all(|w| w[0] == w[1])
    }

    pub fn last_actions(&self) -> Vec<String> {
        self.last_actions.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_detection() {
        let mut detector = LoopDetector::new(10);
        detector.record_action("action_a".to_string());
        detector.record_action("action_b".to_string());
        detector.record_action("action_a".to_string());
        detector.record_action("action_a".to_string());
        detector.record_action("action_a".to_string());

        assert!(detector.is_looping());
    }

    #[test]
    fn test_not_looping() {
        let mut detector = LoopDetector::new(10);
        detector.record_action("action_a".to_string());
        detector.record_action("action_b".to_string());
        detector.record_action("action_c".to_string());

        assert!(!detector.is_looping());
    }
}
