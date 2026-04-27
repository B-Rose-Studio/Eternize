use askama::Template;
use eternize_models::hero::Hero;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/hero.html")]
pub struct HeroSection {
    pub model: Hero,
}
