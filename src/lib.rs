mod markup;

use axum::{http, routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use markup::page_layout;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_service::Service as _;

fn router() -> Router {
    let cors = CorsLayer::new().allow_origin(AllowOrigin::predicate(|origin, _req| {
        // Allow any subdomain from the base
        origin.as_bytes().ends_with(b"joeloach.co.uk")
    }));

    Router::new()
        .route(
            "/",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page_layout(None, markup::home::page(), hx | boosted)
                },
            ),
        )
        .route("/latest", get(|| async move { markup::latest::markup() }))
        .layer(cors)
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
