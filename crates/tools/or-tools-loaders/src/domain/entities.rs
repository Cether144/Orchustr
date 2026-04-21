use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentKind {
    Text,
    Markdown,
    Html,
    Json,
    Csv,
    Pdf,
    Docx,
    Unknown,
}

impl DocumentKind {
    #[must_use]
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "txt" => Self::Text,
            "md" | "mdx" => Self::Markdown,
            "html" | "htm" => Self::Html,
            "json" | "jsonl" => Self::Json,
            "csv" | "tsv" => Self::Csv,
            "pdf" => Self::Pdf,
            "docx" | "doc" => Self::Docx,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoaderRequest {
    /// File path or raw content (base64-encoded for binary).
    pub source: LoaderSource,
    #[serde(default)]
    pub kind_hint: Option<DocumentKind>,
    /// Max characters per chunk (0 = no chunking).
    #[serde(default)]
    pub chunk_size: usize,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LoaderSource {
    Path { path: String },
    Raw { content: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub content: String,
    pub kind: DocumentKind,
    #[serde(default)]
    pub chunk_index: usize,
    #[serde(default)]
    pub metadata: Value,
}

impl Document {
    #[must_use]
    pub fn new(content: impl Into<String>, kind: DocumentKind) -> Self {
        Self {
            content: content.into(),
            kind,
            chunk_index: 0,
            metadata: Value::Null,
        }
    }
}
