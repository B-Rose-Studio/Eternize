use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub section_title: String,
    pub photo_one_url: String,
    pub photo_one_caption: String,
    pub photo_two_url: String,
    pub photo_two_badge_text: String,
    pub photo_two_caption: String,
    pub photo_three_url: String,
    pub photo_three_caption: String,
}
