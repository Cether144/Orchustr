use crate::domain::entities::{Document, LoaderRequest};
use crate::domain::errors::LoaderError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DocumentLoader: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn load(&self, req: LoaderRequest) -> Result<Vec<Document>, LoaderError>;
}
