use askama::Template;
use serde::{Deserialize, Serialize};
use worker::*;

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
pub struct MainPage {
    page_meta_title: String,
    user_ordered_sections: Vec<SectionType>,
}

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let page = MainPage {
        page_meta_title: "Bruno C.".to_string(),
        user_ordered_sections: vec![SectionType::Hero, SectionType::Vault],
    };
    Response::from_html(page.render().unwrap())
}
