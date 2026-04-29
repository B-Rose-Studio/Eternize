use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    id: Uuid,
    name: String,
    order: u8,
}
