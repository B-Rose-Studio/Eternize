use eternize_render::{CustomizePageTemplate, MainPageTemplate, NotFoundTemplate};
use eternize_repository::{DB, Repository, d1::PageD1Repositiry};
use eternize_services::database;
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        // Main Page
        .get_async("/", |_, _| async move {
            return Response::from_html(MainPageTemplate::render());
        })
        // Customize Page
        .get_async("/:page_id", |_, ctx| async move {
            let db = DB::new(&ctx.env, "ETERNIZE-DB").unwrap();

            let page_id = ctx.param("page_id").unwrap().to_string();
            let page_repo = PageD1Repositiry::new(db.as_ref());

            let (page, sections, properties) =
                database::PageServices::get_page_in_chain(page_repo, page_id).await;

            if let Some(page) = page {
                if page.active {
                    return Response::from_html(CustomizePageTemplate::render(
                        page, sections, properties,
                    ));
                }
            }

            return Response::from_html(NotFoundTemplate::render());
        })
        .run(req, env)
        .await
}
