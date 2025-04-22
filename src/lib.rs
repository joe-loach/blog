mod markup;
mod models;

use std::sync::Arc;

use axum::{http, routing::get, Extension, Router};
use markup::{home, post};
use tower_service::Service as _;
use worker::{send::SendWrapper, Bucket, D1Database};

#[derive(Clone)]
pub struct WorkerEnv(pub SendWrapper<worker::Env>);

#[derive(Clone)]
pub struct PostBucket(pub SendWrapper<Bucket>);

#[derive(Clone)]
pub struct PostDB(Arc<D1Database>);

fn router(env: worker::Env) -> Router {
    let bucket = PostBucket(SendWrapper::new(env.bucket("blog_posts").unwrap()));
    let db = PostDB(Arc::new(env.d1("DB").unwrap()));
    let env = WorkerEnv(SendWrapper::new(env));

    Router::new()
        .route("/", get(home::root))
        .nest("/post", post::router())
        .layer(Extension(db))
        .layer(Extension(bucket))
        .layer(Extension(env))
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
