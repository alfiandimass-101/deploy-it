use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ServerSummary {
    pub data: Vec<ServerAttributesContainer>, 
    #[serde(flatten)]
    extra: Value, 
}

#[derive(Debug, Deserialize)]
struct ServerAttributesContainer {
    pub attributes: MinimalAttributes,
    #[serde(flatten)]
    extra: Value, 
}

#[derive(Debug, Deserialize)]
struct MinimalAttributes {
    pub uuid: String,
    pub identifier: String,
    #[serde(flatten)]
    extra: Value, 
}