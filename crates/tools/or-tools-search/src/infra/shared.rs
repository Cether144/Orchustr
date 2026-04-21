#![cfg_attr(
    not(any(
        feature = "tavily",
        feature = "exa",
        feature = "brave",
        feature = "serper",
        feature = "searxng",
        feature = "youcom",
        feature = "bing"
    )),
    allow(dead_code)
)]

use crate::domain::errors::SearchError;
use reqwest::Response;
use url::Url;

pub(crate) fn build_url(
    provider: &'static str,
    base: &str,
    params: &[(&str, &str)],
) -> Result<Url, SearchError> {
    let mut url = Url::parse(base).map_err(|e| SearchError::Transport {
        provider: provider.into(),
        reason: format!("invalid endpoint `{base}`: {e}"),
    })?;
    {
        let mut pairs = url.query_pairs_mut();
        for (k, v) in params {
            pairs.append_pair(k, v);
        }
    }
    Ok(url)
}

pub(crate) async fn decode_response<T: serde::de::DeserializeOwned>(
    provider: &'static str,
    response: Response,
) -> Result<T, SearchError> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(SearchError::Upstream {
            provider: provider.into(),
            status: status.as_u16(),
            body,
        });
    }
    response
        .json::<T>()
        .await
        .map_err(|e| SearchError::Serialization {
            provider: provider.into(),
            reason: e.to_string(),
        })
}

pub(crate) fn transport(provider: &'static str, err: reqwest::Error) -> SearchError {
    SearchError::Transport {
        provider: provider.into(),
        reason: err.to_string(),
    }
}

pub(crate) fn load_api_key(env_var: &'static str) -> Result<String, SearchError> {
    std::env::var(env_var).map_err(|_| SearchError::MissingApiKey(env_var.into()))
}
