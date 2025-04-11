mod markup;

use axum::{http, routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use markup::page_layout;
use maud::html;
use tower_http::cors::{self, AllowOrigin, CorsLayer};
use tower_service::Service as _;

fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _req| {
            // Allow any subdomain from the base
            origin.as_bytes().ends_with(b"joeloach.co.uk")
        }))
        // Allow ALL headers to pass through
        .allow_headers(cors::Any);

    Router::new()
        .route(
            "/",
            get(
                |HxRequest(hx): HxRequest, HxBoosted(boosted): HxBoosted| async move {
                    page_layout(None, markup::home::page(), hx | boosted)
                },
            ),
        )
        .route(
            "/latest",
            get(|HxRequest(hx): HxRequest| async move {
                let content = markup::latest::markup();
                if hx {
                    html! {
                        // add stylesheet to head
                        head hx-head="merge" {
                            link rel="stylesheet" type="text/css" href="https://blog.joeloach.co.uk/tailwind.css";
                        }
                        (content)
                    }
                } else {
                    content
                }
            }).layer(cors),
        )
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
