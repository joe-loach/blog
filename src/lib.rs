mod markup;

use axum::{http, routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use markup::{page_layout, post};
use tower_service::Service as _;

fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page_layout(markup::Title::Top, markup::home::page(), hx | boosted)
                },
            ),
        )
        .nest("/post", post::router())
}

#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}
