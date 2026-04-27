use serde::{Deserialize, Serialize};

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
