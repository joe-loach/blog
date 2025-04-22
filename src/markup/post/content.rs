use axum::{extract::Path, http::StatusCode, Extension};
use axum_extra::{headers::Origin, TypedHeader};
use axum_htmx::{HxBoosted, HxRequest};
use chrono::NaiveDate;
use maud::{html, Markup, PreEscaped};

use crate::{
    markup::{page_layout, Title},
    models::Post,
    PostBucket, PostDB,
};

use super::{style::add_style_if_cors, Metadata};

#[worker::send]
pub async fn get_blog_content(
    Path((year, month, day, title)): Path<(u32, u32, u32, String)>,
    HxRequest(hx): HxRequest,
    HxBoosted(boosted): HxBoosted,
    Extension(bucket): Extension<PostBucket>,
    Extension(db): Extension<PostDB>,
    origin: Option<TypedHeader<Origin>>,
) -> Result<Markup, StatusCode> {
    // work out the key into the bucket
    let date = NaiveDate::from_ymd_opt(year as i32, month, day).ok_or(StatusCode::BAD_REQUEST)?;

    let query =
        db.0.prepare("SELECT meta FROM posts WHERE title = ?1 AND date = ?2")
            .bind(&[
                title.into(),
                date.format(Metadata::DATE_FMT).to_string().into(),
            ])
            .expect("valid sql");

    let Some(post) = query
        .first::<Post>(None)
        .await
        .expect("failed to deserialize meta")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let post_content = bucket
        .0
        .get(post.meta.key)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // extract post body text
    let contents = post_content
        .body()
        .unwrap()
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let page = add_style_if_cors(
        origin.is_some(),
        html! {
            // wrap the content in .post styling
            .post {
                (PreEscaped(contents))
            }
        },
    );

    Ok(page_layout(
        Title::Blog(&post.meta.title),
        page,
        hx && boosted,
    ))
}
