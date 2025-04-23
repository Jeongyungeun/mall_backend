use crate::domain::{
    model::{error::DomainError, item::Item},
    port::{
        primary::item_service_port::ItemServicePort,
        secondary::item_repository_port::ItemRepository,
    },
};
#[derive(Debug, Clone)]
pub struct ItemService<R: ItemRepository> {
    repo: R,
}

impl<R: ItemRepository> ItemService<R> {
    pub fn new(item_repository: R) -> Self {
        Self {
            repo: item_repository,
        }
    }
}

#[async_trait::async_trait]
impl<R: ItemRepository + Send + Sync> ItemServicePort for ItemService<R> {
    async fn create_item(&self, item: Item) -> Result<Item, DomainError> {
        // 비즈니스 로직
        self.repo.save(&item).await?;
        Ok(item)
    }
}
