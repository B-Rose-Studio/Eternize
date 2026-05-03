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
    pub timeline_section_title: String,
    pub timeline_events: Vec<TimelineEvent>,
}

impl TryFrom<&HashMap<String, String>> for Timeline {
    type Error = String;

    fn try_from(map: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let timeline_events = match map.get("timeline_events") {
            Some(json_str) => serde_json::from_str(json_str)
                .map_err(|e| format!("Parse error JSON in 'events': {}", e))?,
            None => Vec::new(),
        };

        Ok(Timeline {
            timeline_section_title: map
                .get("timeline_section_title")
                .cloned()
                .unwrap_or_default(),
            timeline_events,
        })
    }
}
