use chrono::NaiveDate;
use yaml_rust::Yaml;

use crate::models::{self};

use super::tag::Tag;

pub struct Metadata {
    pub title: String,
    pub date: NaiveDate,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub enum ParseMetadataError {
    Title,
    Date,
}

impl Metadata {
    pub const TITLE_KEY: &str = "title";
    pub const DATE_KEY: &str = "date";
    pub const TAGS_KEY: &str = "tags";

    pub const DATE_FMT: &str = "%Y-%m-%d";

    pub fn parse_from_yaml(yaml: &Yaml) -> Result<Self, ParseMetadataError> {
        let title = yaml[Self::TITLE_KEY]
            .as_str()
            .ok_or(ParseMetadataError::Title)?
            .to_owned();

        let date_str = yaml[Self::DATE_KEY]
            .as_str()
            .ok_or(ParseMetadataError::Date)?;
        let date = NaiveDate::parse_from_str(date_str, Self::DATE_FMT)
            .map_err(|_| ParseMetadataError::Date)?;

        let tags = {
            let tags = &yaml[Self::TAGS_KEY];
            match tags {
                Yaml::Array(yamls) => yamls
                    .iter()
                    .filter_map(|yaml| yaml.as_str().map(|s| Tag(s.to_owned())))
                    .collect(),
                _ => Vec::new(),
            }
        };

        Ok(Self { title, date, tags })
    }
}

impl From<models::Post> for Metadata {
    fn from(post: models::Post) -> Self {
        let meta = post.meta;
        Metadata {
            title: meta.title,
            date: NaiveDate::parse_from_str(&meta.date, Self::DATE_FMT)
                .expect("failed to parse date"),
            tags: meta.tags.into_iter().map(Tag).collect(),
        }
    }
}
