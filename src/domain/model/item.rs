use chrono::DateTime;
use chrono::Utc;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;

use super::error::CreateError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemId(Option<String>);

impl ItemId {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<&str> for ItemId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value.to_string())))
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemName(Option<String>);

impl ItemName {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<String> for ItemName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Name is None")
        } else {
            Ok(Self(Some(value)))
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemPrice(Option<u32>);

impl ItemPrice {
    fn value(&self) -> &Option<u32> {
        &self.0
    }
}

impl TryFrom<u32> for ItemPrice {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self(Some(value)))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemImage(Option<Vec<String>>);

impl ItemImage {
    fn value(&self) -> &Option<Vec<String>> {
        &self.0
    }
}

impl TryFrom<Vec<String>> for ItemImage {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    FunctionalFood,
    Otc,
    Etc,
    MedicalDevice,
    Base,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemDescription {
    usage: Option<String>,
}
impl ItemDescription {
    fn usage(&self) -> &Option<String> {
        &self.usage
    }
}

impl TryFrom<&str> for ItemDescription {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(ItemDescription { usage: None })
        } else {
            Ok(ItemDescription {
                usage: Some(value.to_string()),
            })
        }
    }
}

/// builder pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into))]
pub struct Item {
    pub id: ItemId,
    pub name: ItemName,
    pub price: ItemPrice,
    pub item_type: ItemType,
    pub item_images: ItemImage,
    pub description: ItemDescription,
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub updated_at: DateTime<Utc>,
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub created_at: DateTime<Utc>,
}
impl Item {
    pub fn new(
        id: &str,
        name: String,
        price: u32,
        item_type: ItemType,
        item_images: Vec<String>,
        description: &str,
    ) -> Result<Item, CreateError> {
        let id = ItemId::try_from(id)?;
        let name = ItemName::try_from(name)?;
        let price = ItemPrice::try_from(price)?;
        let item_images = ItemImage::try_from(item_images)?;
        let description = ItemDescription::try_from(description)?;

        Ok(Item {
            id,
            name,
            price,
            item_type,
            item_images,
            description,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        })
    }
}
