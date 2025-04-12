mod metadata;
mod style;

use std::path::PathBuf;

use axum::{extract::Path, http::StatusCode, routing::get, Router};
use axum_extra::{headers::Origin, TypedHeader};
use axum_htmx::{HxBoosted, HxRequest};
use chrono::Datelike;
use include_dir::{include_dir, Dir};
use maud::{html, Markup, PreEscaped};
use metadata::{parse_metadata, MetaData};
use pulldown_cmark::Options;
use style::add_style_if_cors;
use tower_http::cors::{self, AllowOrigin, CorsLayer};

use crate::page_layout;

use super::Title;

static POSTS: Dir<'_> = include_dir!("./posts");

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _req| {
            // Allow any subdomain from the base
            origin.as_bytes().ends_with(b"joeloach.co.uk")
        }))
        // Allow ALL headers to pass through
        .allow_headers(cors::Any);

    Router::new()
        .route("/latest", get(get_latest))
        .route("/{year}/{month}/{day}/{name}", get(get_post_content))
        .layer(cors)
}

/// A `Post` that has a `path` to retrieve content from and [`MetaData`].
pub struct PostFile {
    pub path: String,
    pub meta: MetaData,
}

pub fn all() -> Markup {
    html! {
        (post_body(&all_post_data(Order::Descending)))
    }
}

pub fn latest() -> Markup {
    const LATEST_MAX_LEN: usize = 2;

    let mut posts = all_post_data(Order::Descending);
    let n_latest = posts.len().min(LATEST_MAX_LEN);
    let latest = posts.drain(..n_latest).collect::<Vec<_>>();

    html! {
        article class="blog:mt-8 blog:flex blog:flex-col blog:gap-16 blog:pb-16" {
            (post_body(&latest))
        }
    }
}

async fn get_latest(origin: Option<TypedHeader<Origin>>) -> Markup {
    add_style_if_cors(origin.is_some(), latest())
}

async fn get_post_content(
    Path((year, month, day, name)): Path<(u32, u32, u32, PathBuf)>,
    HxRequest(hx): HxRequest,
    HxBoosted(boosted): HxBoosted,
    origin: Option<TypedHeader<Origin>>,
) -> Result<Markup, StatusCode> {
    // add the markdown extension back to the filename
    let file_name = name.with_extension("md");

    let post = POSTS.get_file(file_name).ok_or(StatusCode::NOT_FOUND)?;

    // split metadata
    let (front, post_markdown) =
        matter::matter(post.contents_utf8().unwrap()).expect("failed to read metadata");

    let meta = parse_metadata(&front);
    // ensure the date is correct
    let date = meta.date;
    if !(date.year() == year as i32 && (date.month0() + 1) == month && (date.day0() + 1) == day) {
        return Err(StatusCode::NOT_FOUND);
    }

    // convert markdown into html
    let mut post_html = String::new();
    let parser = pulldown_cmark::Parser::new_ext(&post_markdown, Options::all());
    pulldown_cmark::html::push_html(&mut post_html, parser);

    let page = add_style_if_cors(
        origin.is_some(),
        html! {
            // wrap the content in .post styling
            .post {
                (PreEscaped(post_html))
            }
        },
    );

    Ok(page_layout(Title::Blog(&meta.title), page, hx && boosted))
}

#[allow(unused)]
enum Order {
    Ascending,
    Descending,
}

/// Retrives the data for all posts.
fn all_post_data(order: Order) -> Vec<PostFile> {
    let mut posts = Vec::new();

    for entry in POSTS.find("*.md").unwrap() {
        let include_dir::DirEntry::File(post) = entry else {
            continue;
        };

        let (front, _content) =
            matter::matter(post.contents_utf8().unwrap()).expect("failed to read metadata");

        let meta = parse_metadata(&front);

        posts.push(PostFile {
            // remove extension from path
            path: post.path().with_extension("").to_str().unwrap().to_owned(),
            meta,
        });
    }

    // sort by date
    posts.sort_by_key(|p| p.meta.date);
    // reverse the list if requested to be descending
    if let Order::Descending = order {
        posts.reverse();
    }

    posts
}

fn post_body(posts: &[PostFile]) -> Markup {
    html! {
        @if !posts.is_empty() {
            ul {
                @for post in posts {
                    (post.link())
                }
            }
        } @ else {
            // apologise for not writing
            div {
                h3 ."blog:text-2xl" ."blog:text-foreground" { "No blog posts yet :(" }
                p ."blog:font-light" ."blog:text-accent-foreground" { "Check again back later..." }
            }
        }
    }
}

impl PostFile {
    fn link(&self) -> Markup {
        let human_date = self.meta.date.format("%Y-%m-%d").to_string();
        html! {
            li {
                h3 ."blog:text-2xl" ."blog:text-muted-foreground" { (human_date) }
                a ."blog:hover:underline blog:decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(self.href()) {
                    h2 ."blog:text-4xl" ."blog:font-semibold" ."blog:break-normal" { (self.meta.title) }
                }
            }
        }
    }

    /// `post`/`YYYY`/`MM`/`DD`/`title`
    fn href(&self) -> String {
        let href_date = self.meta.date.format("%Y/%m/%d").to_string();

        format!("/post/{}/{}", href_date, self.path)
    }
}
