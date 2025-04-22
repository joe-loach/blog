use std::{fs, io::Write as _, process::Stdio};

use dotenvy::dotenv;
use glob::glob;

#[test]
fn generate_and_submit_posts() {
    dotenv().expect("should be a `.env` file in project root");

    let auth_key = std::env::var("POST_AUTH_KEY_SECRET").expect("auth key exists");
    let auth_header = format!("x-post-key: {}", auth_key);

    for entry in glob("./posts/*.md").unwrap() {
        let entry = entry.expect("glob error :/");

        eprintln!("Generating: {}", entry.display());

        // curl .../post/create -X PUT --header "X-Post-Key: test" --data-binary @-
        let mut child = std::process::Command::new("curl")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args([
                // "http://localhost:8787/post/create",
                "https://blog.joeloach.co.uk/post/create",
                // method
                "-X",
                "PUT",
                // authorisation
                "--header",
                &auth_header,
                // get file binary data from pipe
                "--data-binary",
                "@-",
            ])
            .spawn()
            .expect("failed to run wrangler");

        // send the file contents through the pipe
        {
            let contents = fs::read_to_string(&entry).expect("should be able to read entry");

            let mut stdin = child.stdin.take().unwrap();
            stdin
                .write_all(contents.as_bytes())
                .expect("failed to write contents to wrangler");
        }

        let output = child
            .wait_with_output()
            .expect("failed to wait on wrangler");

        eprintln!("{}", String::from_utf8(output.stdout).unwrap());
        assert!(output.status.success(), "curl failed");
    }
}
