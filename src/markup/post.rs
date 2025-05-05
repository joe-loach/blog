#[cfg(test)]
mod generate;

mod content;
mod extract;
mod list;
mod manager;
mod metadata;
mod style;
mod tag;

use axum::{routing::get, Router};
use metadata::Metadata;
use tower_http::cors::{self, AllowOrigin, CorsLayer};

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _req| {
            // Allow any subdomain from the base
            origin.as_bytes().ends_with(b"joeloach.co.uk")
        }))
        // Allow ALL headers to pass through
        .allow_headers(cors::Any);

    Router::new()
        .route("/list", get(list::list_all))
        .route("/latest", get(list::list_latest))
        .route("/{y}/{m}/{d}/{name}", get(content::get_blog_content))
        .nest("/manage", manager::router())
        .layer(cors)
}
