#![allow(dead_code)]

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
    pub attributes: MinimalServerAttributes,
    #[serde(flatten)]
    extra: Value, 
}

#[derive(Debug, Deserialize)]
struct MinimalServerAttributes {
    pub uuid: String,
    pub identifier: String,
    #[serde(flatten)]
    extra: Value, 
}