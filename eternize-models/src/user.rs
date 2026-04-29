use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserType {
    Cleint,
    Admin,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub active: bool,
    pub birthdate: DateTime<Utc>,
}
