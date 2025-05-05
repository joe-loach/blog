use axum::{http::StatusCode, Extension};
use axum_extra::{headers::Origin, TypedHeader};
use chrono::NaiveDate;
use maud::{html, Markup};

use crate::{
    markup::{
        post::{style::add_style_if_cors, tag::tags, Metadata},
        LinkStyle,
    },
    models::Post,
    PostDB,
};

#[worker::send]
pub async fn list_all(Extension(db): Extension<PostDB>) -> Result<Markup, StatusCode> {
    let query = db.0.prepare("SELECT id, meta FROM posts ORDER BY date DESC");

    let posts = query
        .all()
        .await
        .expect("query failed")
        .results::<Post>()
        .expect("failed to deserialize post meta");

    let posts = posts.into_iter().map(Metadata::from).collect::<Vec<_>>();

    if posts.is_empty() {
        return Ok(empty_list());
    }

    let items = html! {
        @for (_idx, meta) in posts.iter().enumerate() {
            @let link = link(meta, LinkStyle::Relative);
            li { (link) }
        }
    };

    Ok(html! {
        ul { (items) }
    })
}

#[worker::send]
pub async fn list_latest(
    Extension(db): Extension<PostDB>,
    origin: Option<TypedHeader<Origin>>,
) -> Result<Markup, StatusCode> {
    let query =
        db.0.prepare("SELECT id, meta FROM posts ORDER BY date DESC LIMIT 2");

    let posts = query
        .all()
        .await
        .expect("query failed")
        .results::<Post>()
        .expect("failed to deserialize post meta");

    let posts = posts.into_iter().map(Metadata::from).collect::<Vec<_>>();

    let cors = origin.is_some();

    let style = match cors {
        true => LinkStyle::Absolute,
        false => LinkStyle::Relative,
    };

    let content = if !posts.is_empty() {
        html! {
            ul {
                @for meta in posts {
                    li { (link(&meta, style)) }
                }
            }
        }
    } else {
        empty_list()
    };

    Ok(add_style_if_cors(cors, content))
}

fn link(meta: &Metadata, style: LinkStyle) -> Markup {
    fn href(meta: &Metadata, style: LinkStyle) -> String {
        let location = encode_route(meta.date, &meta.title);
        format!("{}/post/{}", style.base(), location)
    }

    let human_date = meta.date.format("%Y-%m-%d").to_string();
    html! {
        h3 ."blog:text-2xl" ."blog:text-muted-foreground" { (human_date) }
        a ."blog:hover:underline blog:decoration-2" hx-boost="true" hx-target="#content" hx-swap="innerHTML show:no-scroll" href=(href(meta, style)) {
            h2 ."blog:text-4xl" ."blog:font-semibold" ."blog:break-normal" { (meta.title) }
        }
        (tags(&meta.tags))
    }
}

fn encode_route(date: NaiveDate, name: &str) -> String {
    format!("{}/{}", date.format("%Y/%m/%d"), name)
}

fn empty_list() -> Markup {
    html! {
        div {
            h3 ."blog:text-2xl" ."blog:text-foreground" { "No blog posts yet :(" }
            p ."blog:font-light" ."blog:text-accent-foreground" { "Check again back later..." }
        }
    }
}
