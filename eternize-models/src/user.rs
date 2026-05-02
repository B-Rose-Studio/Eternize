use crate::utils::bool_from_int;
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

    #[serde(rename = "type")]
    pub user_type: UserType,

    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub birthdate: DateTime<Utc>,

    #[serde(deserialize_with = "bool_from_int")]
    pub active: bool,
}
