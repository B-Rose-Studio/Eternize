use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub max_sections: u8,
}
