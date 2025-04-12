use chrono::NaiveDate;
use yaml_rust::YamlLoader;

pub struct MetaData {
    pub title: String,
    pub date: NaiveDate,
}

pub fn parse_metadata(front_matter: &str) -> MetaData {
    let docs = YamlLoader::load_from_str(front_matter).expect("failed to load yaml");
    let meta = docs.first().expect("no meta table in yaml");

    let title = meta["title"]
        .as_str()
        .expect("title should be a string")
        .to_owned();

    let date = {
        let date_str = meta["date"].as_str().expect("date should be a string");
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("failed to parse date")
    };

    MetaData { title, date }
}
