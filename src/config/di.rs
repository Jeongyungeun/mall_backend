use sqlx::PgPool;

use crate::{
    application::usecase::{
        item_service::ItemService,
        user_service::{self, UserService},
    },
    driven::{item_repository_impl::ItemRepositoryImpl, user_repository_impl::UserRepositoryImpl},
};

#[derive(Debug, Clone)]
pub struct AppContainer {
    pub item_service: ItemService<ItemRepositoryImpl>,
    pub user_service: UserService<UserRepositoryImpl>,
}

impl AppContainer {
    pub fn new(pool: &PgPool) -> Self {
        let item_repo = ItemRepositoryImpl::new(pool);
        let user_repo = UserRepositoryImpl::new(pool);
        let item_service = ItemService::new(item_repo);
        let user_service = UserService::new(user_repo);

        Self {
            item_service,
            user_service,
        }
    }
}
