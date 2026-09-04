use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BudgetError {
    #[error("Token budget exceeded: {consumed}/{allocated}")]
    TokenBudgetExceeded { consumed: usize, allocated: usize },

    #[error("Action budget exceeded: {consumed}/{allocated}")]
    ActionBudgetExceeded { consumed: usize, allocated: usize },

    #[error("Time budget exceeded: {consumed:?}/{allocated:?}")]
    TimeBudgetExceeded { consumed: Duration, allocated: Duration },
}

pub type BudgetResult<T> = Result<T, BudgetError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub tokens_allocated: usize,
    pub tokens_consumed: usize,
    pub actions_allocated: usize,
    pub actions_consumed: usize,
    pub time_limit_seconds: u64,
}

impl Budget {
    pub fn new(tokens: usize, actions: usize, time_seconds: u64) -> Self {
        Self {
            tokens_allocated: tokens,
            tokens_consumed: 0,
            actions_allocated: actions,
            actions_consumed: 0,
            time_limit_seconds: time_seconds,
        }
    }

    pub fn consume_tokens(&mut self, count: usize) -> BudgetResult<()> {
        self.tokens_consumed += count;
        if self.tokens_consumed > self.tokens_allocated {
            return Err(BudgetError::TokenBudgetExceeded {
                consumed: self.tokens_consumed,
                allocated: self.tokens_allocated,
            });
        }
        Ok(())
    }

    pub fn consume_action(&mut self) -> BudgetResult<()> {
        self.actions_consumed += 1;
        if self.actions_consumed > self.actions_allocated {
            return Err(BudgetError::ActionBudgetExceeded {
                consumed: self.actions_consumed,
                allocated: self.actions_allocated,
            });
        }
        Ok(())
    }

    pub fn remaining_tokens(&self) -> usize {
        self.tokens_allocated.saturating_sub(self.tokens_consumed)
    }

    pub fn remaining_actions(&self) -> usize {
        self.actions_allocated.saturating_sub(self.actions_consumed)
    }

    pub fn token_usage_percent(&self) -> f32 {
        if self.tokens_allocated == 0 {
            0.0
        } else {
            (self.tokens_consumed as f32 / self.tokens_allocated as f32) * 100.0
        }
    }
}

#[derive(Debug)]
pub struct BudgetTracker {
    budget: Budget,
    start_time: Instant,
}

impl BudgetTracker {
    pub fn new(budget: Budget) -> Self {
        Self {
            budget,
            start_time: Instant::now(),
        }
    }

    pub fn consume_tokens(&mut self, count: usize) -> BudgetResult<()> {
        self.budget.consume_tokens(count)
    }

    pub fn consume_action(&mut self) -> BudgetResult<()> {
        self.budget.consume_action()
    }

    pub fn exceeded(&self) -> bool {
        self.budget.tokens_consumed > self.budget.tokens_allocated
            || self.budget.actions_consumed > self.budget.actions_allocated
            || self.elapsed() > Duration::from_secs(self.budget.time_limit_seconds)
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn time_remaining(&self) -> Duration {
        let limit = Duration::from_secs(self.budget.time_limit_seconds);
        let elapsed = self.elapsed();
        limit.saturating_sub(elapsed)
    }

    pub fn budget(&self) -> &Budget {
        &self.budget
    }

    pub fn budget_mut(&mut self) -> &mut Budget {
        &mut self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_token_consumption() {
        let mut budget = Budget::new(1000, 10, 60);
        assert!(budget.consume_tokens(500).is_ok());
        assert_eq!(budget.remaining_tokens(), 500);
        assert!(budget.consume_tokens(500).is_ok());
        assert!(budget.consume_tokens(1).is_err());
    }

    #[test]
    fn test_budget_action_consumption() {
        let mut budget = Budget::new(1000, 5, 60);
        for _ in 0..5 {
            assert!(budget.consume_action().is_ok());
        }
        assert!(budget.consume_action().is_err());
    }

    #[test]
    fn test_budget_tracker_elapsed() {
        let budget = Budget::new(1000, 10, 60);
        let tracker = BudgetTracker::new(budget);
        let elapsed = tracker.elapsed();
        assert!(elapsed < Duration::from_secs(1));
    }
}
