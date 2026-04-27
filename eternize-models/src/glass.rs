use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Glass {
    pub background_image_url: String,
    pub card_one_title: String,
    pub card_one_image_url: String,
    pub card_two_text: String,
    pub card_three_cta_text: String,
    pub card_three_video_url: String,
}
