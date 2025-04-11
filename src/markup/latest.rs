use maud::{html, Markup};

use crate::markup::post::{self, TEST_POST};

pub fn markup() -> Markup {
    html! {
        article class="mt-8 flex flex-col gap-16 pb-16" {
            ul {
                (post::link_item(&TEST_POST))
            }
        }
    }
}
