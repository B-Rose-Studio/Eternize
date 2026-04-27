use askama::Template;
use eternize_models::gift::Gift;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/gift.html")]
pub struct GiftSection {
    pub model: Gift,
}
