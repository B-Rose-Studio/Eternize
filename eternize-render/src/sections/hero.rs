use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeroHighlight {
    pub label: String,
}

#[derive(Template, Debug, Clone)]
#[template(path = "sections/hero.html")]
pub struct HeroSection {
    pub hero_cover_image_url: String,
    pub hero_badge_text: String,
    pub hero_main_title: String,
    pub hero_subtitle_message: String,
    pub hero_highlight_items: Vec<HeroHighlight>,
}
