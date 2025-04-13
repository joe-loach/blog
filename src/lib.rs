mod markup;

use axum::{http, routing::get, Extension, Router};
use markup::{home, page_layout, post};
use tower_service::Service as _;
use worker::{send::SendWrapper, Bucket};

#[derive(Clone)]
pub struct PostBucket(pub SendWrapper<Bucket>);

fn router(env: worker::Env) -> Router {
    let bucket = PostBucket(worker::send::SendWrapper::new(env.bucket("blog_posts").unwrap()));

    Router::new()
        .route("/", get(home::root))
        .nest("/post", post::router())
        .layer(Extension(bucket))
}

#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router(env).call(req).await?)
}
