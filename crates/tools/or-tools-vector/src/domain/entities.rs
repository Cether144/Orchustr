use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Distance {
    Cosine,
    Euclidean,
    DotProduct,
}

impl Distance {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cosine => "cosine",
            Self::Euclidean => "euclidean",
            Self::DotProduct => "dotproduct",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionConfig {
    pub name: String,
    pub dimension: u32,
    #[serde(default = "default_distance")]
    pub distance: Distance,
}

fn default_distance() -> Distance {
    Distance::Cosine
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpsertItem {
    pub id: String,
    pub vector: Vec<f32>,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpsertBatch {
    pub collection: String,
    pub items: Vec<UpsertItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRequest {
    pub collection: String,
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryFilter {
    pub collection: String,
    pub vector: Vec<f32>,
    #[serde(default = "default_top_k")]
    pub top_k: u32,
    #[serde(default)]
    pub filter: Option<Value>,
}

fn default_top_k() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VectorMatch {
    pub id: String,
    pub score: f32,
    #[serde(default)]
    pub metadata: Value,
}
