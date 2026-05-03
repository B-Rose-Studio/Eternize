use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hero {
    pub hero_cover_image_url: String,
    pub hero_badge_text: String,
    pub hero_main_title: String,
    pub hero_subtitle_message: String,
    pub hero_highlight_items: Vec<String>,
}

impl TryFrom<&HashMap<String, String>> for Hero {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let hero_highlight_items = if let Some(items_str) = map.get("hero_highlight_items") {
            serde_json::from_str(items_str)
                .map_err(|e| format!("Parse error JSON in 'highlight_items': {}", e))?
        } else {
            Vec::new()
        };

        Ok(Hero {
            hero_cover_image_url: map.get("hero_cover_image_url").cloned().unwrap_or_default(),
            hero_badge_text: map.get("hero_badge_text").cloned().unwrap_or_default(),
            hero_main_title: map.get("hero_main_title").cloned().unwrap_or_default(),
            hero_subtitle_message: map
                .get("hero_subtitle_message")
                .cloned()
                .unwrap_or_default(),
            hero_highlight_items,
        })
    }
}
