use crate::domain::model::{error::DomainError, item::Item};

#[async_trait::async_trait]
pub trait ItemServicePort {
    async fn create_item(&self, item: Item) -> Result<Item, DomainError>;
    // 필요에 따라 다른 메서드 추가
}
