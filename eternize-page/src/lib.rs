use eternize_render::CustomizePageTemplate;
use worker::*;

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let html = CustomizePageTemplate::test();
    Response::from_html(html)
}
