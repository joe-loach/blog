use maud::{html, Markup};

use crate::markup::post::{latest_posts, post_body};

pub fn markup() -> Markup {
    html! {
        article class="blog:mt-8 blog:flex blog:flex-col blog:gap-16 blog:pb-16" {
            (post_body(&latest_posts()))
        }
    }
}
