use chrono::Utc;
use eternize_models::{customize_page::CustomizePage, section::Section};
use eternize_render::CustomizePageTemplate;
use std::collections::HashMap;
use uuid::Uuid;
use worker::*;

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let params = CustomizePage {
        id: Uuid::new_v4(),
        active: true,
        name: "Teste".into(),
        title: "Teste".into(),
        purchased_in: Utc::now(),
        renewed_in: Utc::now(),
    };

    let sections = vec![
        Section {
            id: Uuid::new_v4(),
            name: "Hero".into(),
            order: 0,
        },
        Section {
            id: Uuid::new_v4(),
            name: "Glass".into(),
            order: 1,
        },
        Section {
            id: Uuid::new_v4(),
            name: "Album".into(),
            order: 2,
        },
        Section {
            id: Uuid::new_v4(),
            name: "Timeline".into(),
            order: 3,
        },
        Section {
            id: Uuid::new_v4(),
            name: "Gift".into(),
            order: 4,
        },
    ];

    let propertys = HashMap::new();

    let html = CustomizePageTemplate::render(params, sections, propertys);
    Response::from_html(html)
}
