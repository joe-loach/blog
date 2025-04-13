use std::{fs, io::Write, process::Stdio};

use chrono::NaiveDate;
use glob::glob;
use pulldown_cmark::Options;
use yaml_rust::YamlLoader;

use crate::markup::post::key::encode_key;

#[test]
fn generate_and_submit_posts() {
    for entry in glob("./posts/*.md").unwrap() {
        let entry = entry.expect("glob error :/");

        eprintln!("Generating: {}", entry.display());

        let contents = fs::read_to_string(&entry).expect("file exists");
        let (yaml, markdown) = matter::matter(&contents).expect("split frontmatter");

        let yaml = YamlLoader::load_from_str(&yaml).expect("valid yaml doc");
        let meta = &yaml[0]; // load up the first (and only) document

        let title = meta["title"].as_str().to_owned().unwrap();
        let date = NaiveDate::parse_from_str(meta["date"].as_str().unwrap(), "%Y-%m-%d")
            .expect("date is valid");

        let parser = pulldown_cmark::Parser::new_ext(&markdown, Options::all());
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        let key = encode_key(date, title);
        let object_path = format!("blog-posts/{key}");

        eprintln!("Creating {}", object_path);

        let mut child = std::process::Command::new("wrangler")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(["r2", "object", "put", &object_path, "--pipe", "--remote"])
            .spawn()
            .expect("failed to run wrangler");

        // write the html to wrangler
        {
            let mut stdin = child.stdin.take().unwrap();
            stdin
                .write_all(html.as_bytes())
                .expect("failed to write contents to wrangler");
        }

        let output = child
            .wait_with_output()
            .expect("failed to wait on wrangler");

        eprintln!("{}", String::from_utf8(output.stdout).unwrap());
        assert!(output.status.success(), "wrangler failed");
    }
}
