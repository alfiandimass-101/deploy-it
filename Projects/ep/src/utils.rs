#![allow(dead_code)]

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct ServerSummary {
    pub data: Vec<ServerAttributesContainer>,
    #[serde(flatten)]
    extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct ServerAttributesContainer {
    pub attributes: MinimalServerAttributes,
    #[serde(flatten)]
    extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct MinimalServerAttributes {
    pub uuid: String,
    pub identifier: String,
    #[serde(flatten)]
    extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct UploaderJson {
    pub object: String,
    pub attributes: UploadAttributes,
}

#[derive(Debug, Deserialize)]
pub struct UploadAttributes {
    pub url: String,
}