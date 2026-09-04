use crate::Plan;

pub struct Replanner;

impl Replanner {
    pub fn adapt_plan(_plan: &Plan, _failed_goal: &str) -> Plan {
        _plan.clone()
    }
}
