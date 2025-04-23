use sqlx::PgPool;

use crate::domain::{model::item::Item, port::secondary::item_repository_port::ItemRepository};

pub struct ItemRepositoryImpl {
    pool: PgPool,
}

impl ItemRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn save(&self, item: &Item) -> Result<(), crate::domain::model::error::DomainError> {
        sqlx::query("INSERT INTO item (id, name) VALUES ($1, $2)")
            .bind(item.id.value())
            .bind(&item.name.value())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: crate::domain::model::item::ItemId,
    ) -> Option<crate::domain::model::item::Item> {
        todo!()
    }

    async fn delete(
        &self,
        id: crate::domain::model::item::ItemId,
    ) -> Result<(), crate::domain::model::error::DomainError> {
        todo!()
    }

    async fn update(
        &self,
        id: crate::domain::model::item::ItemId,
    ) -> Result<(), crate::domain::model::error::DomainError> {
        todo!()
    }

    async fn find_by_query(
        &self,
        query: String,
    ) -> Result<(), crate::domain::model::error::DomainError> {
        todo!()
    }
}
