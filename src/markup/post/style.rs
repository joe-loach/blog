use maud::{html, Markup};

/// Cors requests require styling
pub fn add_style_if_cors(cors: bool, content: Markup) -> Markup {
    if cors {
        style(content)
    } else {
        content
    }
}

pub fn style(content: Markup) -> Markup {
    html! {
        // add stylesheet to head
        head hx-head="append" {
            link rel="stylesheet" type="text/css" href="https://blog.joeloach.co.uk/tailwind.css";
        }
        (content)
    }
}
