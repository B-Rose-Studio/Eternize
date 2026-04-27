use askama::Template;
use eternize_models::glass::Glass;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/glass.html")]
pub struct GlassSection {
    pub model: Glass,
}
