use std::{env, fs, path::{Path, PathBuf, StripPrefixError}};

use chrono::NaiveDate;
use glob::glob;
use pulldown_cmark::{html, Parser};
use yaml_rust::YamlLoader;

const ASSETS: &str = "./public";

struct Meta {
    title: String,
    date: NaiveDate,
}

fn main() {
    println!("cargo::rerun-if-changed=./posts");
    println!("cargo::rerun-if-changed=build.rs");

    let mut post_info = Vec::new();

    // ASSETS/posts
    let post_dir = Path::new(&ASSETS).join("posts");
    // remove the directory before processing
    let _ = fs::remove_dir_all(&post_dir);

    for post in glob("./posts/*.md").expect("correct pattern") {
        let (front, content) = {
            let post = post.unwrap();

            let post_contents = fs::read_to_string(post).expect("failed to read post file");
            matter::matter(&post_contents).expect("failed to read metadata")
        };

        // Convert markdown -> html
        let mut html_contents = String::new();
        let parser = Parser::new(&content);
        html::push_html(&mut html_contents, parser);

        // Parse the front matter into metadata
        let meta = parse_front_matter(&front);

        // Blog posts are stored in:
        // ASSETS/posts/date
        let dir = post_dir.join(meta.date.format("%Y/%m/%d").to_string());
        // make sure the dir is created
        let _ = fs::create_dir_all(&dir);

        // The blog post file is a html file with the `"lower(title)".html` as the name.
        let file = dir.join(format!("{}.html", meta.title.to_lowercase()));
        // Create the file
        fs::write(&file, html_contents).expect("failed to write file");

        // Record info about the file
        post_info.push((meta, file));
    }

    // create the static list
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let generated_file = Path::new(&out_dir).join("all_posts_generated.rs");

    let mut code = "#[allow(unused)] static ALL_POSTS: &[Post] = &[".to_owned();
    for (Meta { title, date }, file) in post_info {
        code.push_str(&format!(
            "Post {{ date: \"{}\", title: \"{}\", page: \"{}\" }}, ",
            date,
            title,
            replace_prefix(file, ASSETS, "/").unwrap().display()
        ));
    }
    code.push_str("];");

    fs::write(&generated_file, code).unwrap();
}

fn parse_front_matter(matter: &str) -> Meta {
    let docs = YamlLoader::load_from_str(matter).expect("failed to load yaml");
    let meta = docs.first().expect("no meta table in yaml");

    let title = meta["title"]
        .as_str()
        .expect("title should be a string")
        .to_owned();

    let date = {
        let date_str = meta["date"].as_str().expect("date should be a string");
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("failed to parse date")
    };

    Meta { title, date }
}

fn replace_prefix(p: impl AsRef<Path>, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<PathBuf, StripPrefixError> {
    p.as_ref().strip_prefix(from).map(|p| to.as_ref().join(p))
}