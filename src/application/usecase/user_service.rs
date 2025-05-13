use crate::domain::{
    model::{error::DomainError, user::User},
    port::{
        primary::user_service_port::UserServicePort,
        secondary::user_repository_port::UserRepository,
    },
};

#[derive(Debug, Clone)]
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            repo: user_repository,
        }
    }
}

#[async_trait::async_trait]
impl<R: UserRepository + Send + Sync> UserServicePort for UserService<R> {
    async fn create_user(&self, user: &User) -> Result<User, DomainError> {
        // 비즈니스 로직 (예: 유효성 검사, 비밀번호 해싱 등)
        todo!()
    }
    async fn update_user(&self, user: &User) -> Result<(), DomainError> {
        todo!()
    }
}
