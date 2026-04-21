#![cfg_attr(
    not(any(
        feature = "pinecone",
        feature = "weaviate",
        feature = "qdrant",
        feature = "chroma",
        feature = "milvus",
        feature = "pgvector"
    )),
    allow(dead_code)
)]

use crate::domain::errors::VectorError;
use reqwest::Response;

pub(crate) async fn decode<T: serde::de::DeserializeOwned>(
    provider: &'static str,
    response: Response,
) -> Result<T, VectorError> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(VectorError::Upstream {
            provider: provider.into(),
            status: status.as_u16(),
            body,
        });
    }
    response
        .json::<T>()
        .await
        .map_err(|e| VectorError::Serialization {
            provider: provider.into(),
            reason: e.to_string(),
        })
}

pub(crate) async fn expect_ok(
    provider: &'static str,
    response: Response,
) -> Result<(), VectorError> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(VectorError::Upstream {
            provider: provider.into(),
            status: status.as_u16(),
            body,
        });
    }
    Ok(())
}

pub(crate) fn transport(provider: &'static str, err: reqwest::Error) -> VectorError {
    VectorError::Transport {
        provider: provider.into(),
        reason: err.to_string(),
    }
}

pub(crate) fn load_credential(env_var: &'static str) -> Result<String, VectorError> {
    std::env::var(env_var).map_err(|_| VectorError::MissingCredential(env_var.into()))
}
