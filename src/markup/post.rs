mod content;
pub mod key;
mod list;
mod style;

use axum::{routing::get, Router};
use chrono::NaiveDate;
use key::decode_key;
use maud::{html, Markup};
use tower_http::cors::{self, AllowOrigin, CorsLayer};
use worker::Object;

use crate::markup::LinkStyle;

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _req| {
            // Allow any subdomain from the base
            origin.as_bytes().ends_with(b"joeloach.co.uk")
        }))
        // Allow ALL headers to pass through
        .allow_headers(cors::Any);

    Router::new()
        .route("/list", get(list::get_all_paged))
        .route("/latest", get(list::get_latest))
        .route("/{y}/{m}/{d}/{name}", get(content::get_blog_content))
        .layer(cors)
}

pub struct BlogPostInfo {
    pub title: String,
    pub date: NaiveDate,
}

impl BlogPostInfo {
    pub fn from_object(obj: Object) -> Self {
        let key = obj.key();
        let (date, title) = decode_key(key);

        BlogPostInfo { title, date }
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
        format!("{}/post/read/{}", style.base(), location)
    }
}

fn encode_route(date: NaiveDate, name: &str) -> String {
    format!("{}/{}", date.format("%Y/%m/%d"), name)
}
