use sqlx::PgPool;

use crate::domain::{
    model::{error::DomainError, user::User},
    port::secondary::user_repository_port::UserRepository,
};

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        todo!()
    }
}
