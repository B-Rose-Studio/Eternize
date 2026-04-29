use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gift {
    pub teaser_text: String,
    pub interaction_instruction: String,
    pub reveal_title: String,
    pub surprise_name: String,
    pub surprise_message: String,
    pub surprise_action_url: String,
    pub surprise_button_label: String,
}

impl TryFrom<&HashMap<String, String>> for Gift {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Gift {
            teaser_text: map.get("teaser_text").cloned().unwrap_or_default(),
            interaction_instruction: map
                .get("interaction_instruction")
                .cloned()
                .unwrap_or_default(),
            reveal_title: map.get("reveal_title").cloned().unwrap_or_default(),
            surprise_name: map.get("surprise_name").cloned().unwrap_or_default(),
            surprise_message: map.get("surprise_message").cloned().unwrap_or_default(),
            surprise_action_url: map.get("surprise_action_url").cloned().unwrap_or_default(),
            surprise_button_label: map
                .get("surprise_button_label")
                .cloned()
                .unwrap_or_default(),
        })
    }
}
