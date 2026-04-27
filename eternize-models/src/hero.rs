use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hero {
    pub cover_image_url: String,
    pub badge_text: String,
    pub main_title: String,
    pub subtitle_message: String,
    pub highlight_items: Vec<String>,
}
