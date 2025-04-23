use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;

use crate::domain::model::error::DomainError;

#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    Database(DatabaseError),
    ExternalService(String),
    Validation(String),
    Authentication(String),
    Authorization(String),
    Other(String),
}

#[derive(Debug)]
pub enum DatabaseError {
    Connection(String),
    Query(String),
    Duplicate(String),
    NotFound(String),
    Other(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Connection(e) => write!(f, "Connection error: {}", e),
            DatabaseError::Query(e) => write!(f, "Query error: {}", e),
            DatabaseError::Duplicate(e) => write!(f, "Duplicate data: {}", e),
            DatabaseError::NotFound(e) => write!(f, "Data not found: {}", e),
            DatabaseError::Other(e) => write!(f, "Other database error: {}", e),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Domain(e) => write!(f, "Domain error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::ExternalService(e) => write!(f, "External service error: {}", e),
            AppError::Validation(e) => write!(f, "Validation error: {}", e),
            AppError::Authentication(e) => write!(f, "Authentication error: {}", e),
            AppError::Authorization(e) => write!(f, "Authorization error: {}", e),
            AppError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
impl std::error::Error for DatabaseError {}
impl std::error::Error for DomainError {}

impl From<SqlxError> for DatabaseError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::Database(e) => {
                if let Some(code) = e.code() {
                    match code.as_ref() {
                        // 중복 키 에러 (unique violation)
                        "23505" => DatabaseError::Duplicate(e.message().to_string()),
                        // 외래 키 제약 조건 위반
                        "23503" => {
                            DatabaseError::Query(format!("Foreign key violation: {}", e.message()))
                        }
                        // 기타 데이터베이스 에러
                        _ => DatabaseError::Query(e.message().to_string()),
                    }
                } else {
                    DatabaseError::Other(e.message().to_string())
                }
            }
            SqlxError::RowNotFound => {
                DatabaseError::NotFound("Requested data not found".to_string())
            }
            SqlxError::PoolTimedOut => {
                DatabaseError::Connection("Database connection pool timeout".to_string())
            }
            _ => DatabaseError::Other(error.to_string()),
        }
    }
}

impl From<DatabaseError> for DomainError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound(e) => DomainError::SaveError(format!("Not found:{}", e)),
            DatabaseError::Duplicate(e) => DomainError::SaveError(format!("Duplicate:{}", e)),
            _ => DomainError::SaveError(error.to_string()),
        }
    }
}

impl From<SqlxError> for DomainError {
    fn from(error: SqlxError) -> Self {
        // error.into().into() 는 안된다.
        let db_error: DatabaseError = error.into();
        db_error.into()
    }
}

impl From<DatabaseError> for AppError {
    fn from(error: DatabaseError) -> Self {
        AppError::Database(error)
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        let db_error: DatabaseError = error.into();
        AppError::Database(db_error)
    }
}

// DomainError를 AppError로 변환
impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        AppError::Domain(error)
    }
}
// Actix Web과의 통합을 위한 ResponseError 구현
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Domain(DomainError::SaveError(e)) => {
                HttpResponse::BadRequest().json(json_error("Save error", e))
            }
            AppError::Domain(DomainError::DeleteError(e)) => {
                HttpResponse::BadRequest().json(json_error("Delete error", e))
            }
            AppError::Database(DatabaseError::NotFound(e)) => {
                HttpResponse::NotFound().json(json_error("Not found", e))
            }
            AppError::Database(DatabaseError::Duplicate(e)) => {
                HttpResponse::Conflict().json(json_error("Duplicate data", e))
            }
            AppError::Database(DatabaseError::Query(e)) => {
                HttpResponse::BadRequest().json(json_error("Query error", e))
            }
            AppError::Database(DatabaseError::Connection(e)) => {
                HttpResponse::ServiceUnavailable().json(json_error("Database connection error", e))
            }
            AppError::Database(DatabaseError::Other(e)) => {
                HttpResponse::InternalServerError().json(json_error("Database error", e))
            }
            AppError::Validation(e) => {
                HttpResponse::BadRequest().json(json_error("Validation error", e))
            }
            AppError::Authentication(e) => {
                HttpResponse::Unauthorized().json(json_error("Authentication error", e))
            }
            AppError::Authorization(e) => {
                HttpResponse::Forbidden().json(json_error("Authorization error", e))
            }
            AppError::ExternalService(e) => {
                HttpResponse::BadGateway().json(json_error("External service error", e))
            }
            AppError::Other(e) => {
                HttpResponse::InternalServerError().json(json_error("Internal server error", e))
            }
        }
    }
}

// JSON 에러 응답 생성 헬퍼 함수
fn json_error(error_type: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": {
            "type": error_type,
            "message": message
        }
    })
}
