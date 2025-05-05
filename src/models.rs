use serde::{de::Error, Serialize};
use serde::{Deserialize, Deserializer};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub date: String,
    pub key: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct PostId(pub usize);

#[derive(Deserialize)]
pub struct Post {
    pub id: PostId,
    #[serde(deserialize_with = "from_json")]
    pub meta: Metadata,
}

fn from_json<'de, D>(deserializer: D) -> Result<Metadata, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    let result = serde_json::from_str(&buf).map_err(D::Error::custom)?;

    Ok(result)
}
