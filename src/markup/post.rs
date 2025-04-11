use chrono::NaiveDate;
use maud::{html, Markup};

include!(concat!(env!("OUT_DIR"), "/all_posts_generated.rs"));

pub fn all_posts() -> &'static [Post] {
    ALL_POSTS
}

pub fn latest_posts() -> Vec<Post> {
    const LATEST_MAX_LEN: usize = 2;

    let mut posts = all_posts().to_vec();

    // sort them by recency
    posts.sort_unstable_by_key(|post| {
        NaiveDate::parse_from_str(post.date, "%Y-%m-%d").expect("internal date, should parse")
    });
    posts.reverse();

    let n_latest = posts.len().min(LATEST_MAX_LEN);
    posts.drain(..n_latest).collect()
}

pub fn post_body(posts: &[Post]) -> Markup {
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

#[derive(Clone)]
pub struct Post {
    pub date: &'static str,
    pub title: &'static str,
    pub page: &'static str,
}

impl Post {
    pub fn link(&self) -> Markup {
        html! {
            li {
                h3 ."blog:text-2xl" ."blog:text-muted-foreground" { (self.date) }
                a ."blog:hover:underline blog:decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(self.page) {
                    h2 ."blog:text-4xl" ."blog:font-semibold" ."blog:break-normal" { (self.title) }
                }
            }
        }
    }
}
