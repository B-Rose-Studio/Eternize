use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub name: String,
    pub order: u8,
    pub page_id: Uuid,
}
