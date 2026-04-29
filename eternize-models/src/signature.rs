use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    id: Uuid,
    name: String,
    price: f64,
    max_sections: u8,
}
