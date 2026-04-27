use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/hero.html")]
pub struct HeroSection {}
