use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use serde::Deserialize;

use crate::{
    models::{Metadata as ModelMetadata, Post, PostId},
    PostBucket, PostDB,
};

use super::{
    extract::{CreatePostHeader, CreatePostSecret},
    tag::Tag,
    Metadata,
};

pub fn router() -> Router {
    Router::new().route(
        "/",
        post(create_post).patch(update_post).delete(delete_post),
    )
}

#[worker::send]
async fn create_post(
    Extension(bucket): Extension<PostBucket>,
    Extension(db): Extension<PostDB>,
    provided_key: CreatePostHeader,
    post_key_secret: CreatePostSecret,
    body: String,
) -> Result<String, StatusCode> {
    if !is_authorized(provided_key, post_key_secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (meta, html) = parse_post_body(&body).ok_or(StatusCode::BAD_REQUEST)?;

    // put the blog content into the bucket
    let object = bucket
        .0
        .put(&meta.title, html)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let meta_json = create_json_metadata(meta, &object);

    // add the metadata to the database
    db.0.prepare("INSERT INTO posts (meta) VALUES (?)")
        .bind(&[meta_json.into()])
        .expect("valid sql")
        .run()
        .await
        .expect("inserted post sucessfully");

    Ok(object.key())
}

#[derive(Deserialize)]
struct Update {
    post_id: PostId,
    content: String,
}

#[worker::send]
async fn update_post(
    Extension(bucket): Extension<PostBucket>,
    Extension(db): Extension<PostDB>,
    provided_key: CreatePostHeader,
    post_key_secret: CreatePostSecret,
    Json(Update { post_id, content }): Json<Update>,
) -> Result<(), StatusCode> {
    if !is_authorized(provided_key, post_key_secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let Some(post) =
        db.0.prepare("SELECT id, meta FROM posts WHERE id = 1?")
            .bind(&[post_id.0.into()])
            .expect("valid sql")
            .first::<Post>(None)
            .await
            .expect("failed to deserialize")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    // delete the old bucket data
    // FIXME: check this error?
    let _err = bucket.0.delete(&post.meta.key).await;

    let (meta, html) = parse_post_body(&content).ok_or(StatusCode::BAD_REQUEST)?;

    // put the blog content into the bucket
    let new_object = bucket
        .0
        .put(&meta.title, html)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let meta_json = create_json_metadata(meta, &new_object);

    // update the database with new information
    db.0.prepare("UPDATE posts SET meta = ?1 WHERE id = ?2")
        .bind(&[meta_json.into(), post.id.0.into()])
        .expect("valid sql")
        .run()
        .await
        .expect("updated post sucessfully");

    Ok(())
}

#[derive(Deserialize)]
struct Delete {
    post_id: PostId,
}

#[worker::send]
async fn delete_post(
    Extension(bucket): Extension<PostBucket>,
    Extension(db): Extension<PostDB>,
    provided_key: CreatePostHeader,
    post_key_secret: CreatePostSecret,
    Json(Delete { post_id }): Json<Delete>,
) -> Result<(), StatusCode> {
    if !is_authorized(provided_key, post_key_secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // make sure post exists
    let Some(post) =
        db.0.prepare("SELECT id, meta FROM posts WHERE id = 1?")
            .bind(&[post_id.0.into()])
            .expect("valid sql")
            .first::<Post>(None)
            .await
            .expect("failed to deserialize")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    // delete the post
    db.0.prepare("DELETE FROM posts WHERE id = ?1")
        .bind(&[post_id.0.into()])
        .expect("valid sql")
        .run()
        .await
        .expect("deleted post sucessfully");

    // and its content
    bucket
        .0
        .delete(post.meta.key)
        .await
        .expect("deleted post content");

    Ok(())
}

fn parse_post_body(body: &str) -> Option<(Metadata, String)> {
    use yaml_rust::YamlLoader;

    let (yaml, markdown) = matter::matter(body).expect("split frontmatter");

    let yaml = YamlLoader::load_from_str(&yaml).expect("valid yaml doc");

    let meta = &yaml[0]; // load up the first (and only) document
    let meta = Metadata::parse_from_yaml(meta).ok()?;

    let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    Some((meta, html))
}

fn create_json_metadata(meta: Metadata, object: &worker::Object) -> String {
    let meta = ModelMetadata {
        title: meta.title,
        date: meta.date.format(Metadata::DATE_FMT).to_string(),
        key: object.key(),
        tags: meta.tags.into_iter().map(|Tag(tag)| tag).collect(),
    };

    serde_json::to_string(&meta).expect("should serialize")
}

fn is_authorized(
    CreatePostHeader(provided_key): CreatePostHeader,
    CreatePostSecret(post_key_secret): CreatePostSecret,
) -> bool {
    provided_key == post_key_secret.as_bytes()
}
