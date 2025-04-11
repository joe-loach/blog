use maud::{html, Markup};

use crate::markup::post::{all_posts, post_body};

pub fn page() -> Markup {
    html! {
        article class="blog:mt-8 blog:flex blog:flex-col blog:gap-16 blog:pb-16" {
            (post_body(all_posts()))
        }
    }
}
