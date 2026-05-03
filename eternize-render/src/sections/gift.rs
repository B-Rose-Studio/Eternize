use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gift {
    pub gift_teaser_text: String,
    pub gift_interaction_instruction: String,
    pub gift_reveal_title: String,
    pub gift_surprise_name: String,
    pub gift_surprise_message: String,
    pub gift_surprise_action_url: String,
    pub gift_surprise_button_label: String,
}

impl TryFrom<&HashMap<String, String>> for Gift {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Gift {
            gift_teaser_text: map.get("gift_teaser_text").cloned().unwrap_or_default(),
            gift_interaction_instruction: map
                .get("gift_interaction_instruction")
                .cloned()
                .unwrap_or_default(),
            gift_reveal_title: map.get("gift_reveal_title").cloned().unwrap_or_default(),
            gift_surprise_name: map.get("gift_surprise_name").cloned().unwrap_or_default(),
            gift_surprise_message: map
                .get("gift_surprise_message")
                .cloned()
                .unwrap_or_default(),
            gift_surprise_action_url: map
                .get("gift_surprise_action_url")
                .cloned()
                .unwrap_or_default(),
            gift_surprise_button_label: map
                .get("gift_surprise_button_label")
                .cloned()
                .unwrap_or_default(),
        })
    }
}
