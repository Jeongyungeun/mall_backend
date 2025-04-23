use crate::domain::model::{
    error::DomainError,
    item::{Item, ItemId},
};

#[async_trait::async_trait]
pub trait ItemRepository {
    async fn save(&self, item: &Item) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: ItemId) -> Option<Item>;
    async fn delete(&self, id: ItemId) -> Result<(), DomainError>;
    async fn update(&self, id: ItemId) -> Result<(), DomainError>;
    async fn find_by_query(&self, query: String) -> Result<(), DomainError>;
}
