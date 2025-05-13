use crate::domain::model::{error::DomainError, user::User};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}
