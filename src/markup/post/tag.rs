use maud::{html, Markup, Render};

pub fn tags<'a>(tags: impl IntoIterator<Item = &'a Tag>) -> Markup {
    html! {
        ul class="blog:flex blog:flex-row blog:flex-wrap blog:items-start blog:gap-1 blog:pb-2 blog:list-none" {
            @for tag in tags.into_iter() {
                li { (tag) }
            }
        }
    }
}

pub struct Tag(pub String);

impl Render for Tag {
    fn render(&self) -> Markup {
        html! {
            div class="blog:inline-flex blog:items-center blog:rounded-md blog:font-semibold blog:bg-secondary blog:text-secondary-foreground blog:hover:bg-secondary/80 blog:px-1 blog:py-0 blog:text-[10px]" {
                (self.0)
            }
        }
    }
}
