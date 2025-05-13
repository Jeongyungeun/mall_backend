use crate::domain::model::{error::DomainError, user::User};

#[async_trait::async_trait]
pub trait UserServicePort {
    async fn create_user(&self, user: &User) -> Result<User, DomainError>;
    async fn update_user(&self, user: &User) -> Result<(), DomainError>;
}
