use chrono::Utc;
use eternize_models::{customize_page::CustomizePage, section::Section};
use eternize_render::CustomizePageTemplate;
use eternize_repository::DB;
use std::collections::HashMap;
use uuid::Uuid;
use worker::*;

#[event(fetch)]
async fn fetch(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let _db = DB::new(&env, "ETERNIZE-DB").unwrap();

    let page_id = Uuid::new_v4();
    let params = CustomizePage {
        id: page_id.clone(),
        active: true,
        name: "Teste".into(),
        title: "Teste".into(),
        purchased_in: Utc::now(),
        renewed_in: Utc::now(),
        user_id: Uuid::new_v4(),
        signature_id: Uuid::new_v4(),
    };

    let sections = vec![
        Section {
            id: Uuid::new_v4(),
            name: "Hero".into(),
            order: 0,
            page_id: page_id.clone(),
        },
        Section {
            id: Uuid::new_v4(),
            name: "Glass".into(),
            order: 1,
            page_id: page_id.clone(),
        },
        Section {
            id: Uuid::new_v4(),
            name: "Album".into(),
            order: 3,
            page_id: page_id.clone(),
        },
        Section {
            id: Uuid::new_v4(),
            name: "Timeline".into(),
            order: 2,
            page_id: page_id.clone(),
        },
        Section {
            id: Uuid::new_v4(),
            name: "Gift".into(),
            order: 4,
            page_id: page_id.clone(),
        },
    ];

    let propertys = HashMap::new();

    let html = CustomizePageTemplate::render(params, sections, propertys);
    Response::from_html(html)
}
