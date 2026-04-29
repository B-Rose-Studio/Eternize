use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Glass {
    pub background_image_url: String,
    pub card_one_title: String,
    pub card_one_image_url: String,
    pub card_two_text: String,
    pub card_three_cta_text: String,
    pub card_three_video_url: String,
}

impl TryFrom<&HashMap<String, String>> for Glass {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Glass {
            background_image_url: map.get("background_image_url").cloned().unwrap_or_default(),
            card_one_title: map.get("card_one_title").cloned().unwrap_or_default(),
            card_one_image_url: map.get("card_one_image_url").cloned().unwrap_or_default(),
            card_two_text: map.get("card_two_text").cloned().unwrap_or_default(),
            card_three_cta_text: map.get("card_three_cta_text").cloned().unwrap_or_default(),
            card_three_video_url: map.get("card_three_video_url").cloned().unwrap_or_default(),
        })
    }
}
