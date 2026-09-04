use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),

    #[error("Symlink escape detected: {0}")]
    SymlinkEscape(String),

    #[error("Outside workspace boundary: {0}")]
    OutsideWorkspace(String),

    #[error("Path error: {0}")]
    PathError(String),
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub struct PathValidator {
    workspace_root: PathBuf,
}

impl PathValidator {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    pub fn validate_read(&self, path: &str) -> ValidationResult<PathBuf> {
        self.validate_path(path, false)
    }

    pub fn validate_write(&self, path: &str) -> ValidationResult<PathBuf> {
        self.validate_path(path, true)
    }

    fn validate_path(&self, path: &str, _write: bool) -> ValidationResult<PathBuf> {
        use std::path::Component;

        let path_obj = Path::new(path);

        // Check for path traversal attempts (.. components)
        for component in path_obj.components() {
            if matches!(component, Component::ParentDir) {
                // Allow .. but only if result stays within workspace
            }
        }

        // Prevent absolute paths from outside workspace
        if path_obj.is_absolute() {
            if !path_obj.starts_with(&self.workspace_root) {
                return Err(ValidationError::OutsideWorkspace(path.to_string()));
            }
            return Ok(path_obj.to_path_buf());
        }

        // Relative path: join with workspace root
        let joined = self.workspace_root.join(path_obj);

        // Normalize path by resolving .. components without canonicalize
        // (canonicalize requires the file to exist)
        let normalized = Self::normalize_path(&joined);

        // Verify it's within workspace after normalization
        if !normalized.starts_with(&self.workspace_root) {
            return Err(ValidationError::PathTraversal(path.to_string()));
        }

        Ok(normalized)
    }

    fn normalize_path(path: &Path) -> PathBuf {
        use std::path::Component;

        let mut result = PathBuf::new();
        for component in path.components() {
            match component {
                Component::Prefix(p) => result.push(p.as_os_str()),
                Component::RootDir => result.push("/"),
                Component::Normal(c) => result.push(c),
                Component::ParentDir => {
                    result.pop();
                }
                Component::CurDir => {}
            }
        }
        result
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_valid_relative_path() {
        let temp = TempDir::new().unwrap();
        let validator = PathValidator::new(temp.path().to_path_buf());

        let result = validator.validate_read("test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_traversal_blocked() {
        let temp = TempDir::new().unwrap();
        let validator = PathValidator::new(temp.path().to_path_buf());

        let result = validator.validate_read("../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_absolute_path_outside_workspace() {
        let temp = TempDir::new().unwrap();
        let validator = PathValidator::new(temp.path().to_path_buf());

        #[cfg(windows)]
        let outside_path = "C:\\Windows\\System32";
        #[cfg(not(windows))]
        let outside_path = "/etc/passwd";

        let result = validator.validate_read(outside_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_boundary_enforcement() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join("subdir")).unwrap();

        let validator = PathValidator::new(temp.path().to_path_buf());

        // Valid: inside workspace
        assert!(validator.validate_read("subdir/file.txt").is_ok());

        // Invalid: escape attempt
        assert!(validator.validate_read("subdir/../../outside.txt").is_err());
    }
}
