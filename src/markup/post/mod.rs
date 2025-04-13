mod style;

use axum::{
    extract::{OriginalUri, Path, Query},
    http::StatusCode,
    routing::get,
    Extension, Router,
};
use axum_extra::{headers::Origin, TypedHeader};
use axum_htmx::{HxBoosted, HxRequest};
use chrono::{Datelike, NaiveDate};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use style::add_style_if_cors;
use tower_http::cors::{self, AllowOrigin, CorsLayer};
use worker::Object;

use crate::{page_layout, PostBucket};

use super::Title;

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _req| {
            // Allow any subdomain from the base
            origin.as_bytes().ends_with(b"joeloach.co.uk")
        }))
        // Allow ALL headers to pass through
        .allow_headers(cors::Any);

    Router::new()
        .route("/list", get(get_list))
        .route("/latest", get(get_latest))
        .route("/{y}/{m}/{d}/{name}", get(get_post_content))
        .layer(cors)
}

pub struct PostInfo {
    pub title: String,
    pub date: NaiveDate,
}

#[derive(Deserialize)]
struct Pagination {
    // tells the list cursor where to continue from
    token: Option<String>,
}

#[worker::send]
async fn get_list(
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
        .map(PostInfo::from_object)
        .collect::<Vec<_>>();

    if posts.is_empty() {
        return Ok(no_items());
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
async fn get_latest(
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
        .map(PostInfo::from_object)
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
        no_items()
    };

    Ok(add_style_if_cors(cors, content))
}

fn no_items() -> Markup {
    html! {
        div {
            h3 ."blog:text-2xl" ."blog:text-foreground" { "No blog posts yet :(" }
            p ."blog:font-light" ."blog:text-accent-foreground" { "Check again back later..." }
        }
    }
}

#[worker::send]
async fn get_post_content(
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

    let info = PostInfo::from_object(post);

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

#[allow(unused)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Clone, Copy)]
pub enum LinkStyle {
    Relative,
    Absolute,
}

impl PostInfo {
    pub fn from_object(obj: Object) -> Self {
        let mut meta = obj
            .custom_metadata()
            .expect("every post object has metadata");

        PostInfo {
            title: meta.remove("title").expect("has title"),
            date: NaiveDate::parse_from_str(&meta.remove("date").expect("has date"), "%Y-%m-%d")
                .expect("date is in YYYY-MM-DD format"),
        }
    }

    fn link(&self, style: LinkStyle) -> Markup {
        let human_date = self.date.format("%Y-%m-%d").to_string();
        html! {
            h3 ."blog:text-2xl" ."blog:text-muted-foreground" { (human_date) }
            a ."blog:hover:underline blog:decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(self.href(style)) {
                h2 ."blog:text-4xl" ."blog:font-semibold" ."blog:break-normal" { (self.title) }
            }
        }
    }

    fn href(&self, style: LinkStyle) -> String {
        let location = encode_route(self.date, &self.title);
        let base = match style {
            LinkStyle::Relative => "",
            LinkStyle::Absolute => "https://blog.joeloach.co.uk",
        };
        format!("{}/post/read/{}", base, location)
    }
}

/// ASCD_DATE_NAME
fn encode_key(date: NaiveDate, name: &str) -> String {
    // convert the date into something lexicographically ascending
    let year = 9999 - date.year();
    let month = 99 - (date.month0() + 1);
    let day = 99 - (date.day0() + 1);
    let numbers = format!("{year}-{month}-{day}");

    const NUMBER_TO_DIGIT_DIFFERENCE: u32 = 17;
    let ascending_date = numbers
        .chars()
        .map(|c| {
            if c.is_numeric() {
                char::from_u32(c as u32 + NUMBER_TO_DIGIT_DIFFERENCE).unwrap()
            } else {
                c
            }
        })
        .collect::<String>();

    format!("{}_{}_{}", ascending_date, date.format("%Y-%m-%d"), name)
}

fn encode_route(date: NaiveDate, name: &str) -> String {
    format!("{}/{}", date.format("%Y/%m/%d"), name)
}
