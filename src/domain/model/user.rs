use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserId(pub String);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Self {
        Self(email.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UserRole {
    Customer,
    Admin,
    Staff,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Deleted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Address {
    pub id: String,
    pub name: String,
    pub recipient: String, //수령인
    pub postal_code: String,
    pub address1: String,
    pub address2: String,
    pub city: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PaymentMethod {
    pub id: String,
    pub name: String,
    pub type_: String, //"credit_card", "bank_transfer"
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NotificationSettings {
    pub email_enabled: String,
    pub sms_enabled: String,
    pub push_enabled: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    //식별 정보
    pub id: UserId,
    pub email: Email,
    pub password_hash: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,

    pub name: String,
    pub birth_date: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub profile_image: Option<String>,
    // 결제 관련
    pub address: Vec<Address>,
    pub payment_methods: Vec<PaymentMethod>,
    pub notification_settings: NotificationSettings,
    pub marketing_consent: bool,
    // 쇼핑관련
    pub cart_id: Option<String>,
    pub wishlist_ids: Vec<String>,
    pub loyalty_points: u32,
    //시간관련
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
