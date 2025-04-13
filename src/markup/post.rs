mod content;
mod list;
mod style;

use axum::{routing::get, Router};
use chrono::{Datelike, NaiveDate};
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
        let mut meta = obj
            .custom_metadata()
            .expect("every post object has metadata");
        BlogPostInfo {
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
        format!("{}/post/read/{}", style.base(), location)
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
