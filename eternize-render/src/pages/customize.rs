use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SectionType {
    Hero,
    Album,
    Gala,
    Timeline,
    Glass,
    Vault,
}

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct CustomizePage {
    pub page_meta_title: String,
    pub user_ordered_sections: Vec<SectionType>,
}
