use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub year: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeline {
    pub section_title: String,
    pub events: Vec<TimelineEvent>,
}

impl TryFrom<&HashMap<String, String>> for Timeline {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let events = match map.get("events") {
            Some(json_str) => serde_json::from_str(json_str)
                .map_err(|e| format!("Parse error JSON in 'events': {}", e))?,
            None => Vec::new(),
        };

        Ok(Timeline {
            section_title: map.get("section_title").cloned().unwrap_or_default(),
            events,
        })
    }
}
