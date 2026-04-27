use askama::Template;
use eternize_models::timeline::Timeline;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/timeline.html")]
pub struct TimelineSection {
    pub model: Timeline,
}
