use crate::domain::errors::ProductivityError;
use url::Url;

pub(crate) fn load_credential(env_var: &str) -> Result<String, ProductivityError> {
    std::env::var(env_var).map_err(|_| ProductivityError::MissingCredential(env_var.into()))
}

pub(crate) fn transport(e: reqwest::Error) -> ProductivityError {
    ProductivityError::Transport(e.to_string())
}

pub(crate) fn build_url(base: &str, params: &[(&str, &str)]) -> Result<Url, ProductivityError> {
    let mut url = Url::parse(base).map_err(|e| ProductivityError::Transport(e.to_string()))?;
    for (k, v) in params {
        url.query_pairs_mut().append_pair(k, v);
    }
    Ok(url)
}
