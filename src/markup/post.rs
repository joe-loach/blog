#[cfg(test)]
mod generate;

mod content;
mod create;
mod extract;
pub mod key;
mod list;
mod metadata;
mod style;
mod tag;

use axum::{
    routing::{get, put},
    Router,
};
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
        .route("/list", get(list::get_all_paged))
        .route("/latest", get(list::get_latest))
        .route("/create", put(create::create_new_post))
        .route("/{y}/{m}/{d}/{name}", get(content::get_blog_content))
        .layer(cors)
}
