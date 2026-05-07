use crate::utils::bool_from_int;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThemePage {
    MidnightBlue,
    RoseGold,
    Emerald,
    Obsidian,
    Burgundy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomizePage {
    pub id: Uuid,
    pub theme: ThemePage,
    pub name: String,
    pub title: String,
    pub purchased_in: DateTime<Utc>,
    pub renewed_in: DateTime<Utc>,

    #[serde(deserialize_with = "bool_from_int")]
    pub active: bool,

    pub background_music: String,

    pub user_id: Uuid,
    pub signature_id: Uuid,
}
