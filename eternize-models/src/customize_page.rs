use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomizePage {
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub purchased_in: DateTime<Utc>,
    pub renewed_in: DateTime<Utc>,
    pub active: bool,
}
