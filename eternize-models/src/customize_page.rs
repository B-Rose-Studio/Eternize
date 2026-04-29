use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomizePage {
    id: Uuid,
    name: String,
    title: String,
    purchased_in: DateTime<Utc>,
    renewed_in: DateTime<Utc>,
    active: bool,
}
