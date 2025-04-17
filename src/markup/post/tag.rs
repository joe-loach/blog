use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Tag(pub String);

impl Render for Tag {
    fn render(&self) -> Markup {
        html! {
            // TODO: makle html for tag
        }
    }
}