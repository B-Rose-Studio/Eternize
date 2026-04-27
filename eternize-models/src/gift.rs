use serde::{Deserialize, Serialize};

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
