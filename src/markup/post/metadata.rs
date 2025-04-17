use std::collections::HashMap;

use chrono::NaiveDate;
use yaml_rust::Yaml;

use super::tag::Tag;

pub struct Metadata {
    pub title: String,
    pub date: NaiveDate,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub enum ParseMetadataError {
    CustomMeta,
    Title,
    Date,
    Tags,
}

impl Metadata {
    const TITLE_KEY: &str = "title";
    const DATE_KEY: &str = "date";
    const TAGS_KEY: &str = "tags";

    const DATE_FMT: &str = "%Y-%m-%d";

    pub fn parse_from_yaml(yaml: &Yaml) -> Result<Self, ParseMetadataError> {
        let title = yaml[Self::TITLE_KEY].as_str().ok_or(ParseMetadataError::Title)?.to_owned();

        let date_str = yaml[Self::DATE_KEY].as_str().ok_or(ParseMetadataError::Date)?;
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

    pub fn parse_from_object(object: worker::Object) -> Result<Self, ParseMetadataError> {
        let meta_pairs = object.custom_metadata().map_err(|_| ParseMetadataError::CustomMeta)?;

        let title = meta_pairs
            .get(Self::TITLE_KEY)
            .ok_or(ParseMetadataError::Title)?
            .to_owned();

        let date_str = meta_pairs.get(Self::DATE_KEY).ok_or(ParseMetadataError::Date)?;
        let date =
            NaiveDate::parse_from_str(date_str, Self::DATE_FMT).map_err(|_| ParseMetadataError::Date)?;

        let tags_str = meta_pairs.get(Self::TAGS_KEY).ok_or(ParseMetadataError::Tags)?;
        let tags = serde_json::from_str(tags_str).map_err(|_| ParseMetadataError::Tags)?;

        Ok(Self { title, date, tags })
    }

    pub fn into_hashmap(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(Self::TITLE_KEY.into(), self.title);
        map.insert(
            Self::DATE_KEY.into(),
            self.date.format(Self::DATE_FMT).to_string(),
        );
        map.insert(
            Self::TAGS_KEY.into(),
            serde_json::to_string(&self.tags).expect("should serialize Tags"),
        );
        map
    }
}
