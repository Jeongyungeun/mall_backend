use sqlx::PgPool;

use crate::{
    application::usecase::item::ItemService, driven::item_repository_impl::ItemRepositoryImpl,
};

#[derive(Debug, Clone)]
pub struct AppContainer {
    pub item_service: ItemService<ItemRepositoryImpl>,
}

impl AppContainer {
    pub fn new(pool: PgPool) -> Self {
        let repo = ItemRepositoryImpl::new(pool);
        let item_service = ItemService::new(repo);

        Self { item_service }
    }
}
