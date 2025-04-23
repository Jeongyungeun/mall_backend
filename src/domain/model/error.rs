use core::fmt;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}
impl CreateError {
    pub fn invalid(msg: impl Into<String>) -> Self {
        CreateError::InvalidData(msg.into())
    }
}
impl From<&str> for CreateError {
    fn from(msg: &str) -> Self {
        CreateError::InvalidData(msg.into())
    }
}

#[derive(Debug)]
pub enum DomainError {
    SaveError(String),
    DeleteError(String),
}

impl DomainError {
    pub fn save(msg: impl Into<String>) -> Self {
        DomainError::SaveError(msg.into())
    }
    pub fn delete(msg: impl Into<String>) -> Self {
        DomainError::DeleteError(msg.into())
    }
}
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::DeleteError(e) => write!(f, "Item delete error:{}", e),
            DomainError::SaveError(e) => write!(f, "Item save error:{}", e),
        }
    }
}
