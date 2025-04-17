use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderName, StatusCode},
};

use crate::WorkerEnv;

pub static CREATE_POST_KEY: HeaderName = HeaderName::from_static("x-post-key");

pub struct CreatePostHeader(pub Vec<u8>);

impl<S> FromRequestParts<S> for CreatePostHeader
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(key) = parts.headers.get(&CREATE_POST_KEY) {
            let key = key.as_bytes().to_vec();

            return Ok(CreatePostHeader(key));
        }

        Err((StatusCode::UNAUTHORIZED, "`x-post-key` header is missing"))
    }
}

pub struct CreatePostSecret(pub String);

const AUTH_KEY_SECRET_BINDING: &str = "POST_AUTH_KEY_SECRET";

impl<S> FromRequestParts<S> for CreatePostSecret
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let env = parts
            .extensions
            .get::<WorkerEnv>()
            .expect("WorkerEnv should always be in extensions");

        let secret = env
            .0
            .secret(AUTH_KEY_SECRET_BINDING)
            .expect("`POST_AUTH_KEY_SECRET` exists");

        Ok(CreatePostSecret(secret.to_string()))
    }
}
