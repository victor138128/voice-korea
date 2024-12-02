use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub r#type: String,
    pub id: String,
    pub gsi1: String,
    pub email: String,
    pub password: String,
    pub created_at: i64,
    pub updated_at: Option<u64>,
    pub deleted_at: Option<u64>,
}

impl User {
    pub fn new(email: String, hashed_pw: String) -> Self {
        Self {
            r#type: "user".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            gsi1: User::gsi1(email.clone()),
            email,
            password: hashed_pw,
            created_at: Utc::now().timestamp(),
            updated_at: None,
            deleted_at: None,
        }
    }
    pub fn gsi1(email: String) -> String {
        format!("user#{}", email)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthDocument {
    pub r#type: String,
    pub id: String,
    pub email: String,
    pub value: String,
    pub created_at: i64,
    pub deleted_at: Option<u64>,
}

impl AuthDocument {
    pub fn new(email: String, random_value: String) -> Self {
        Self {
            r#type: "verify".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            email,
            value: random_value,
            created_at: Utc::now().timestamp(),
            deleted_at: None,
        }
    }
}
