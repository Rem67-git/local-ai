use crate::validator::PathValidator;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Invalid workspace: {0}")]
    InvalidWorkspace(String),
}

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;

pub struct WorkspaceManager {
    root: PathBuf,
    validator: PathValidator,
}

impl WorkspaceManager {
    pub fn new(root: PathBuf) -> WorkspaceResult<Self> {
        if !root.is_dir() {
            return Err(WorkspaceError::InvalidWorkspace(format!(
                "Workspace root does not exist: {}",
                root.display()
            )));
        }

        Ok(Self {
            validator: PathValidator::new(root.clone()),
            root,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn validator(&self) -> &PathValidator {
        &self.validator
    }

    pub fn size(&self) -> u64 {
        Self::compute_size(&self.root)
    }

    fn compute_size(path: &Path) -> u64 {
        use std::fs;

        let mut size = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    size += metadata.len();
                    if metadata.is_dir() {
                        size += Self::compute_size(&entry.path());
                    }
                }
            }
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_workspace_creation() {
        let temp = TempDir::new().unwrap();
        let workspace = WorkspaceManager::new(temp.path().to_path_buf()).unwrap();

        assert_eq!(workspace.root(), temp.path());
    }

    #[test]
    fn test_workspace_invalid_path() {
        let result = WorkspaceManager::new(PathBuf::from("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_size() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("file.txt"), "hello world").unwrap();

        let workspace = WorkspaceManager::new(temp.path().to_path_buf()).unwrap();
        assert!(workspace.size() > 0);
    }

    #[test]
    fn test_workspace_validator_accessible() {
        let temp = TempDir::new().unwrap();
        let workspace = WorkspaceManager::new(temp.path().to_path_buf()).unwrap();

        let validator = workspace.validator();
        assert!(validator.validate_read("test.txt").is_ok());
    }
}
