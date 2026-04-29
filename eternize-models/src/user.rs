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
    id: Uuid,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    phone: String,
    active: bool,
    birthdate: DateTime<Utc>,
}
