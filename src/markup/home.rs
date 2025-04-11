use maud::{html, Markup};

use crate::markup::post::{self, TEST_POST};

pub fn page() -> Markup {
    html! {
        article class="blog:mt-8 blog:flex blog:flex-col blog:gap-16 blog:pb-16" {
            ul {
                (post::link_item(&TEST_POST))
            }
        }
    }
}