use crate::state::{MissionState, MissionStatus};
use std::collections::HashMap;

pub struct MissionManager {
    missions: HashMap<String, MissionState>,
}

impl MissionManager {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
        }
    }

    pub fn create_mission(&mut self, goal: String) -> String {
        let mission = MissionState::new(goal);
        let id = mission.id.clone();
        self.missions.insert(id.clone(), mission);
        id
    }

    pub fn get_mission(&self, id: &str) -> Option<&MissionState> {
        self.missions.get(id)
    }

    pub fn get_mission_mut(&mut self, id: &str) -> Option<&mut MissionState> {
        self.missions.get_mut(id)
    }

    pub fn list_missions(&self) -> Vec<&MissionState> {
        self.missions.values().collect()
    }

    pub fn list_active_missions(&self) -> Vec<&MissionState> {
        self.missions
            .values()
            .filter(|m| m.status == MissionStatus::InProgress || m.status == MissionStatus::Pending)
            .collect()
    }

    pub fn remove_mission(&mut self, id: &str) -> Option<MissionState> {
        self.missions.remove(id)
    }
}

impl Default for MissionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_manager_create() {
        let mut manager = MissionManager::new();
        let id = manager.create_mission("Test mission".to_string());
        assert!(manager.get_mission(&id).is_some());
    }

    #[test]
    fn test_mission_manager_list() {
        let mut manager = MissionManager::new();
        manager.create_mission("Mission 1".to_string());
        manager.create_mission("Mission 2".to_string());
        assert_eq!(manager.list_missions().len(), 2);
    }
}
