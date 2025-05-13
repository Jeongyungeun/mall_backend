use chrono::{DateTime, Utc};
use derive_builder::Builder;
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
pub struct User {
    //식별 정보
    pub id: UserId,
    pub email: Email,
    pub password_hash: String, // middle constraint.
    pub phone: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,

    //개인정보
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

impl User {
    pub fn update_user_email(&mut self, new_email: &str) {
        self.email = Email::new(new_email);
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Datelike, TimeZone, Utc};

    use crate::domain::model::user::{
        Address, Email, Gender, NotificationSettings, PaymentMethod, UserId, UserRole, UserStatus,
    };

    use super::{User, UserBuilder};
    #[test]
    fn test_create_user() {
        let now = Utc::now();
        let address = Address {
            id: "addr-001".to_string(),
            name: "집".to_string(),
            recipient: "홍길동".to_string(),
            postal_code: "12345".to_string(),
            address1: "서울시 강남구".to_string(),
            address2: "초전동 해로모 204-2103".to_string(),
            city: Some("서울".to_string()),
            is_default: true,
        };

        let payment_method = PaymentMethod {
            id: "pm-001".to_string(),
            name: "신한카드".to_string(),
            type_: "credit_card".to_string(),
            is_default: true,
        };

        let notification_settings = NotificationSettings {
            email_enabled: "true".to_string(),
            sms_enabled: "true".to_string(),
            push_enabled: "true".to_string(),
        };

        let user = UserBuilder::default()
            .id(UserId::new())
            .email(Email::new("test@example.com"))
            .password_hash("hashed_password".to_string())
            .phone(Some("010-1234-5678".to_string()))
            .role(UserRole::Customer)
            .status(UserStatus::Active)
            .name("홍길동".to_string())
            .birth_date(Some(Utc.with_ymd_and_hms(1982, 7, 14, 12, 0, 0).unwrap()))
            .gender(Some(Gender::Male))
            .profile_image(Some("profile.jpg".to_string()))
            .address(vec![address])
            .payment_methods(vec![payment_method])
            .notification_settings(notification_settings)
            .marketing_consent(true)
            .cart_id(None)
            .wishlist_ids(vec![])
            .loyalty_points(0)
            .last_login_at(Some(now))
            .created_at(now)
            .updated_at(now)
            .build()
            .unwrap();

        assert_eq!(user.email.value(), "test@example.com");
        assert_eq!(user.birth_date.unwrap().year(), 1982);
        assert_eq!(user.birth_date.unwrap().month(), 7);
        assert_eq!(user.birth_date.unwrap().day(), 14);
    }
}
