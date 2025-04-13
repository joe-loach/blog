use axum_htmx::{HxBoosted, HxRequest};
use maud::{html, Markup};

use super::{page_layout, Title};

pub async fn root(
    HxRequest(hx): HxRequest,
    HxBoosted(boosted): HxBoosted,
) -> Markup {
    page_layout(
        Title::Top,
        html! {
            article class="blog:mt-8 blog:flex blog:flex-col blog:gap-16 blog:pb-16" {
                // get the list of blog posts
                div hx-get="/post/list" hx-trigger="load" hx-swap="outerHTML" {}
            }
        },
        hx | boosted,
    )
}
