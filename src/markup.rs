pub mod home;
pub mod post;

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

#[allow(unused)]
pub enum Title<'a> {
    Top,
    Child(&'a str),
    Blog(&'a str),
}

fn format_title(title: Title) -> String {
    match title {
        Title::Top => "Joe's Blog".to_owned(),
        Title::Child(name) => format!("Joe's Blog - {}", name),
        Title::Blog(name) => name.to_owned(),
    }
}

/// Page layout
pub fn page_layout(title: Title, content: Markup, partial: bool) -> Markup {
    if partial {
        return html! {
            title { (format_title(title)) }
            (content)
        };
    }

    html! {
        (head(title))
        body class="blog:mx-auto blog:flex blog:min-h-screen blog:max-w-3xl blog:flex-col blog:px-8" {
            (header())
            main ."blog:grow" {
                div #content {
                    (content)
                }
            }
            (footer())
        }
    }
}

fn head(title: Title) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1";

            meta name="author" content="Joe Loach";
            meta name="description" content="Joes Blog website";
            link rel="canonical" href="https://blog.joeloach.co.uk";

            // Icons
            // link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon/favicon-32x32.png";
            // link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon/favicon-16x16.png";

            // tailwind
            link rel="stylesheet" type="text/css" href="/tailwind.css";

            // Page title
            title { (format_title(title)) }

            // HTMX
            script src="/htmx.min.js" {}

            // // Theme updating
            (Javascript(include_str!("theming.js")))
        }
    }
}

fn header() -> Markup {
    html! {
        header class="blog:sticky blog:top-0 blog:z-40 blog:bg-background/75 blog:py-6 blog:backdrop-blur-sm"{
            nav class="blog:flex blog:items-center blog:justify-between" {
                (nav_links())
                div class="blog:flex blog:gap-0 blog:sm:gap-4" {
                    (theme_toggle())
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer class="blog:flex blog:flex-col blog:items-center blog:justify-center blog:pb-16 blog:sm:flex-row-reverse blog:sm:justify-between" {
            div {}
            p ."blog:text-center" ."blog:text-xs" ."blog:text-muted-foreground" {
                a ."blog:hover:text-foreground" href="https://github.com/joe-loach/blog" { "Hand written html" } " by Joe Loach"
            }
        }
    }
}

fn theme_toggle() -> Markup {
    html! {
        button
          class="blog:cursor-pointer blog:inline-flex blog:items-center blog:justify-center blog:whitespace-nowrap blog:rounded-md blog:text-sm blog:font-medium blog:hover:bg-accent blog:hover:text-accent-foreground blog:h-9 blog:w-9"
          title="Toggle theme"
          aria-label="Toggle theme"
          onClick="toggleTheme()"
        {
            div class="blog:size-4 blog:dark:hidden blog:text-indigo-500" {
                (PreEscaped(iconify::svg!("lucide:moon", width="16px", height="16px")))
            }
            div class="blog:size-4 blog:hidden blog:dark:block blog:text-orange-300"{
                (PreEscaped(iconify::svg!("lucide:sun", width="16px", height="16px")))
            }
        }
    }
}

fn nav_links() -> Markup {
    let link = |name: &str, href: &str| -> Markup {
        html! {
            li ."blog:text-muted-foreground" ."blog:hover:text-foreground" {
                a href=(href) { (name) }
            }
        }
    };

    html! {
        ul hx-boost="true" hx-target="#content" hx-swap="innerHTML show:none" class="blog:flex blog:gap-4 blog:sm:gap-8" {
            (link("home", "/"))
            (link("portfolio", "https://joeloach.co.uk"))
        }
    }
}

pub struct Javascript(&'static str);

impl Render for Javascript {
    fn render(&self) -> Markup {
        html! {
            script type="text/javascript" {
                (PreEscaped(self.0))
            }
        }
    }
}
