use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hero {
    pub cover_image_url: String,
    pub badge_text: String,
    pub main_title: String,
    pub subtitle_message: String,
    pub highlight_items: Vec<String>,
}

impl TryFrom<&HashMap<String, String>> for Hero {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let highlight_items = if let Some(items_str) = map.get("highlight_items") {
            serde_json::from_str(items_str)
                .map_err(|e| format!("Parse error JSON in 'highlight_items': {}", e))?
        } else {
            Vec::new()
        };

        Ok(Hero {
            cover_image_url: map.get("cover_image_url").cloned().unwrap_or_default(),
            badge_text: map.get("badge_text").cloned().unwrap_or_default(),
            main_title: map.get("main_title").cloned().unwrap_or_default(),
            subtitle_message: map.get("subtitle_message").cloned().unwrap_or_default(),
            highlight_items,
        })
    }
}
