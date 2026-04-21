#![cfg_attr(
    not(any(
        feature = "playwright",
        feature = "brightdata",
        feature = "hyperbrowser",
        feature = "agentql",
        feature = "oxylabs"
    )),
    allow(dead_code)
)]

use crate::domain::entities::HttpMethod;
use crate::domain::errors::WebError;
use reqwest::Response;
use std::collections::HashMap;

pub(crate) fn to_reqwest_method(m: HttpMethod) -> reqwest::Method {
    match m {
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
        HttpMethod::Patch => reqwest::Method::PATCH,
        HttpMethod::Delete => reqwest::Method::DELETE,
    }
}

pub(crate) async fn response_to_fetch(
    provider: &'static str,
    response: Response,
) -> Result<crate::domain::entities::FetchResponse, WebError> {
    let status = response.status().as_u16();
    let final_url = response.url().to_string();
    let mut headers = HashMap::new();
    for (name, value) in response.headers().iter() {
        if let Ok(v) = value.to_str() {
            headers.insert(name.to_string(), v.to_string());
        }
    }
    let body = response.text().await.map_err(|e| WebError::Transport {
        provider: provider.into(),
        reason: e.to_string(),
    })?;
    Ok(crate::domain::entities::FetchResponse {
        status,
        body,
        headers,
        final_url: Some(final_url),
    })
}

pub(crate) fn transport(provider: &'static str, err: reqwest::Error) -> WebError {
    WebError::Transport {
        provider: provider.into(),
        reason: err.to_string(),
    }
}

pub(crate) fn load_credential(env_var: &'static str) -> Result<String, WebError> {
    std::env::var(env_var).map_err(|_| WebError::MissingCredential(env_var.into()))
}
