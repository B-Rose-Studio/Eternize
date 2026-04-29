use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl TryFrom<&HashMap<String, String>> for Album {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Album {
            section_title: map.get("section_title").cloned().unwrap_or_default(),
            photo_one_url: map.get("photo_one_url").cloned().unwrap_or_default(),
            photo_one_caption: map.get("photo_one_caption").cloned().unwrap_or_default(),
            photo_two_url: map.get("photo_two_url").cloned().unwrap_or_default(),
            photo_two_badge_text: map.get("photo_two_badge_text").cloned().unwrap_or_default(),
            photo_two_caption: map.get("photo_two_caption").cloned().unwrap_or_default(),
            photo_three_url: map.get("photo_three_url").cloned().unwrap_or_default(),
            photo_three_caption: map.get("photo_three_caption").cloned().unwrap_or_default(),
        })
    }
}
