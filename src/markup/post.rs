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
            h3 ."blog:text-2xl" ."blog:text-muted-foreground" { (post.date) }
            a ."blog:hover:underline blog:decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(post.page) {
                h2 ."blog:text-4xl" ."blog:font-semibold" ."blog:break-normal" { (post.title) }
            }
        }
    }
}
