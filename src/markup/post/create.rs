use axum::{http::StatusCode, Extension};
use yaml_rust::YamlLoader;

use crate::{models::Metadata as ModelMetadata, PostBucket, PostDB};

use super::{
    extract::{CreatePostHeader, CreatePostSecret},
    tag::Tag,
    Metadata,
};

#[worker::send]
pub async fn create_post(
    Extension(bucket): Extension<PostBucket>,
    Extension(db): Extension<PostDB>,
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

    // put the blog content into the bucket
    let object = bucket
        .0
        .put(&meta.title, html)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // create the metadata
    let meta = ModelMetadata {
        title: meta.title,
        date: meta.date.format(Metadata::DATE_FMT).to_string(),
        key: object.key(),
        tags: meta.tags.into_iter().map(|Tag(tag)| tag).collect(),
    };
    let meta_json = serde_json::to_string(&meta).expect("should serialize");

    // add the metadata to the database
    db.0.prepare("INSERT INTO posts (meta) VALUES (?)")
        .bind(&[meta_json.into()])
        .expect("valid sql")
        .run()
        .await
        .expect("inserted post sucessfully");

    Ok(object.key())
}
