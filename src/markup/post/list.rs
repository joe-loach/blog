use axum::{
    extract::{OriginalUri, Query},
    http::StatusCode,
    Extension,
};
use axum_extra::{headers::Origin, TypedHeader};
use maud::{html, Markup};
use serde::Deserialize;

use crate::{
    markup::{
        post::{style::add_style_if_cors, BlogPostInfo},
        LinkStyle,
    },
    PostBucket,
};

#[derive(Deserialize)]
pub struct Pagination {
    // tells the list cursor where to continue from
    token: Option<String>,
}

#[worker::send]
pub async fn get_all_paged(
    Query(Pagination { token }): Query<Pagination>,
    Extension(bucket): Extension<PostBucket>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<Markup, StatusCode> {
    const POSTS_PER_PAGE_LIMIT: u32 = 10;

    let mut list = bucket.0.list().limit(POSTS_PER_PAGE_LIMIT);

    if let Some(token) = &token {
        list = list.cursor(token);
    }

    let objects = list
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let posts = objects
        .objects()
        .into_iter()
        .map(BlogPostInfo::from_object)
        .collect::<Vec<_>>();

    if posts.is_empty() {
        return Ok(empty_list());
    }

    let is_last_post = {
        let last = posts.len();
        move |n: usize| n == (last - 1)
    };

    let query = if let Some(token) = &token {
        format!("?token={}", token)
    } else {
        "".to_string()
    };

    let items = html! {
        @for (idx, post) in posts.iter().enumerate() {
            @let link = post.link(LinkStyle::Relative);
            @if is_last_post(idx) && objects.truncated() {
                // let the last item ask for more if the list was truncated
                li hx-get={(original_uri)(query)} hx-trigger="revealed" hx-swap="afterend" { (link) }
            } @ else {
                li { (link) }
            }
        }
    };

    // on the first request of the list (no pagination), return the list body
    let needs_list_body = token.is_none();
    Ok(html! {
        @if needs_list_body {
            ul { (items) }
        } @else {
            (items)
        }
    })
}

#[worker::send]
pub async fn get_latest(
    Extension(bucket): Extension<PostBucket>,
    origin: Option<TypedHeader<Origin>>,
) -> Result<Markup, StatusCode> {
    const LATEST_POST_COUNT: u32 = 2;

    let list = bucket.0.list().limit(LATEST_POST_COUNT);

    let objects = list
        .execute()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let posts = objects
        .objects()
        .into_iter()
        .map(BlogPostInfo::from_object)
        .collect::<Vec<_>>();

    let cors = origin.is_some();

    let style = match cors {
        true => LinkStyle::Absolute,
        false => LinkStyle::Relative,
    };

    let content = if !posts.is_empty() {
        html! {
            ul {
                @for post in posts{
                    li { (post.link(style)) }
                }
            }
        }
    } else {
        empty_list()
    };

    Ok(add_style_if_cors(cors, content))
}

fn empty_list() -> Markup {
    html! {
        div {
            h3 ."blog:text-2xl" ."blog:text-foreground" { "No blog posts yet :(" }
            p ."blog:font-light" ."blog:text-accent-foreground" { "Check again back later..." }
        }
    }
}
