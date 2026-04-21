use crate::domain::errors::CommsError;

pub(crate) fn load_credential(env_var: &str) -> Result<String, CommsError> {
    std::env::var(env_var).map_err(|_| CommsError::MissingCredential(env_var.into()))
}

pub(crate) fn transport(e: reqwest::Error) -> CommsError {
    CommsError::Transport(e.to_string())
}
