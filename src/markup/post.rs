use maud::{html, Markup};

pub(crate) const TEST_POST: Post = Post {
    date: "Jan 1, 2025",
    title: "Blog Post Test",
    page: "2025/01/01/blog_test.html",
};

// TODO: collect posts from assets
pub struct Post {
    pub date: &'static str,
    pub title: &'static str,
    pub page: &'static str,
}

pub fn link_item(post: &Post) -> Markup {
    html! {
        li {
            h3 ."text-2xl" ."text-muted-foreground" { (post.date) }
            a ."hover:underline decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(post.page) {
                h2 ."text-4xl" ."font-semibold" ."break-normal" { (post.title) }
            }
        }
    }
}
