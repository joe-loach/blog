use axum::{http::StatusCode, Extension};
use yaml_rust::YamlLoader;

use crate::PostBucket;

use super::{
    extract::{CreatePostSecret, CreatePostHeader},
    key::encode_key,
    Metadata,
};

#[worker::send]
pub async fn create_new_post(
    Extension(bucket): Extension<PostBucket>,
    CreatePostHeader(provided_key): CreatePostHeader,
    CreatePostSecret(post_key_secret): CreatePostSecret,
    body: String,
) -> Result<String, StatusCode> {
    if provided_key != post_key_secret.as_bytes() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (yaml, markdown) = matter::matter(&body).expect("split frontmatter");

    let yaml = YamlLoader::load_from_str(&yaml).expect("valid yaml doc");

    let meta = &yaml[0]; // load up the first (and only) document
    let meta = Metadata::parse_from_yaml(meta).map_err(|_| StatusCode::BAD_REQUEST)?;

    let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let key = encode_key(&meta.date, &meta.title);

    let custom_meta_map = meta.into_hashmap();

    let object = bucket
        .0
        .put(key, html)
        .custom_metadata(custom_meta_map)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(object.key())
}
