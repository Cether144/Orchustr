use crate::domain::entities::{
    CollectionConfig, DeleteRequest, QueryFilter, UpsertBatch, VectorMatch,
};
use crate::domain::errors::VectorError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait VectorStoreClient: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    async fn ensure_collection(&self, cfg: CollectionConfig) -> Result<(), VectorError>;

    async fn upsert(&self, batch: UpsertBatch) -> Result<(), VectorError>;

    async fn delete(&self, req: DeleteRequest) -> Result<(), VectorError>;

    async fn query(&self, filter: QueryFilter) -> Result<Vec<VectorMatch>, VectorError>;
}
