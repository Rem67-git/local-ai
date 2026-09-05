use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error("No suitable specialist found")]
    NoSpecialistFound,

    #[error("Delegation failed: {0}")]
    DelegationFailed(String),

    #[error("Specialist error: {0}")]
    SpecialistError(String),

    #[error("Coordination error: {0}")]
    CoordinationError(String),
}

pub type SupervisorResult<T> = Result<T, SupervisorError>;
