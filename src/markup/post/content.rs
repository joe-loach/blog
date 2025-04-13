use axum::{extract::Path, http::StatusCode, Extension};
use axum_extra::{headers::Origin, TypedHeader};
use axum_htmx::{HxBoosted, HxRequest};
use chrono::NaiveDate;
use maud::{html, Markup, PreEscaped};

use crate::{
    markup::{page_layout, Title},
    PostBucket,
};

use super::{encode_key, style::add_style_if_cors, BlogPostInfo};

#[worker::send]
pub async fn get_blog_content(
    Path((year, month, day, name)): Path<(u32, u32, u32, String)>,
    HxRequest(hx): HxRequest,
    HxBoosted(boosted): HxBoosted,
    Extension(bucket): Extension<PostBucket>,
    origin: Option<TypedHeader<Origin>>,
) -> Result<Markup, StatusCode> {
    // work out the key into the bucket
    let date = NaiveDate::from_ymd_opt(year as i32, month, day).ok_or(StatusCode::BAD_REQUEST)?;
    let key = encode_key(date, &name);

    let post = bucket
        .0
        .get(key)
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // extract post body text
    let contents = post
        .body()
        .unwrap()
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let info = BlogPostInfo::from_object(post);

    let page = add_style_if_cors(
        origin.is_some(),
        html! {
            // wrap the content in .post styling
            .post {
                (PreEscaped(contents))
            }
        },
    );

    Ok(page_layout(Title::Blog(&info.title), page, hx && boosted))
}
