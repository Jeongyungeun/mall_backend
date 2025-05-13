use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{item::ItemId, user::UserId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
    pub id: CartId,
    pub user_id: Option<UserId>,
    pub items: HashMap<ItemId, u32>,
    pub items_price: Option<f64>,
    pub total_price: Option<f64>,
    pub status: CartStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct CartId(pub String);

impl CartId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CartId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("cart id error")
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CartStatus {
    Active,
    Abandoned,
    Checkout,
    Completed,
}

impl Cart {
    pub fn new(user_id: Option<UserId>) -> Self {
        let now = Utc::now();

        Self {
            id: CartId::new(),
            user_id,
            items: HashMap::new(),
            items_price: Some(0.0),
            total_price: Some(0.0),
            status: CartStatus::Active,
            expires_at: Some(now + chrono::Duration::days(60)),
            created_at: now,
            updated_at: now,
        }
    }

    //함수형 스타일에서는 불변형을... 러스트의 일반적인 패턴은 가변으로 사용
    pub fn add_item(&mut self, item_id: ItemId, quantity: u32) {
        let current_quantity = self.items.get(&item_id).unwrap_or(&0);
        self.items.insert(item_id, current_quantity + quantity);
        self.updated_at = Utc::now();
    }

    pub fn remove_item(&mut self, item_id: &ItemId) -> bool {
        let removed = self.items.remove(item_id).is_some();
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }
    // item_id를 참조 형식으로 사용해도 되나?
    pub fn update_quantity(&mut self, item_id: &ItemId, quantity: u32) -> bool {
        if quantity == 0 {
            return self.remove_item(item_id);
        }

        if self.items.contains_key(item_id) {
            self.items.insert(item_id.clone(), quantity);
            self.updated_at = Utc::now();
            return true;
        }
        false
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.updated_at = Utc::now();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn total_items(&self) -> u32 {
        self.items.values().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::model::item::ItemId;

    fn create_test_cart() -> Cart {
        Cart::new(None)
    }

    fn create_test_item_id(id: &str) -> ItemId {
        ItemId::try_from(id).unwrap()
    }
    #[test]
    fn test_new_cart() {
        let cart = create_test_cart();

        assert!(cart.user_id.is_none());
        assert!(cart.items.is_empty());
        assert_eq!(cart.total_price, Some(0.0));
        assert_eq!(cart.status, CartStatus::Active);
        assert!(cart.expires_at.is_some());
    }

    #[test]
    fn test_add_item() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        cart.add_item(item_id.clone(), 2);
        assert_eq!(cart.items.get(&item_id), Some(&2));
        assert_eq!(cart.item_count(), 1);
        assert_eq!(cart.total_items(), 2);
    }

    #[test]
    fn test_add_item_increases_quantity() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        cart.add_item(item_id.clone(), 2);
        cart.add_item(item_id.clone(), 3);
        assert_eq!(cart.items.get(&item_id), Some(&5));
    }

    #[test]
    fn test_remove_item() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        cart.add_item(item_id.clone(), 2);
        assert_eq!(cart.item_count(), 1);

        let removed = cart.remove_item(&item_id);
        assert!(removed);
        assert_eq!(cart.item_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        let removed = cart.remove_item(&item_id);
        assert!(!removed);
    }

    #[test]
    fn test_update_quantity() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        cart.add_item(item_id.clone(), 1);

        let updated = cart.update_quantity(&item_id, 2);

        assert!(updated);
        assert_eq!(cart.items.get(&item_id), Some(&2));
    }
    #[test]
    fn test_zero_item_update_quantity() {
        let mut cart = create_test_cart();
        let item_id = create_test_item_id("item1");

        cart.add_item(item_id.clone(), 1);

        let updated = cart.update_quantity(&item_id, 0);
        assert!(updated);
        assert_eq!(cart.items.get(&item_id), None);
    }

    #[test]
    fn test_clear_cart() {
        let mut cart = create_test_cart();
        let item_id1 = create_test_item_id("item1");
        let item_id2 = create_test_item_id("item2");

        cart.add_item(item_id1.clone(), 1);
        cart.add_item(item_id2.clone(), 2);

        assert_eq!(cart.item_count(), 2);

        cart.clear();

        assert!(cart.is_empty());
        assert_eq!(cart.item_count(), 0);
        assert_eq!(cart.total_items(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut cart = create_test_cart();
        assert!(cart.is_empty());

        cart.add_item(create_test_item_id("item1"), 1);

        assert!(!cart.is_empty());
    }

    #[test]
    fn test_item_count() {
        let mut cart = create_test_cart();

        assert_eq!(cart.item_count(), 0);

        cart.add_item(create_test_item_id("item1"), 1);
        cart.add_item(create_test_item_id("item2"), 2);

        assert_eq!(cart.item_count(), 2);
    }

    #[test]
    fn test_total_item() {
        let mut cart = create_test_cart();

        assert_eq!(cart.total_items(), 0);

        cart.add_item(create_test_item_id("item1"), 2);
        cart.add_item(create_test_item_id("item2"), 3);

        assert_eq!(cart.total_items(), 5);
    }
}
