use crate::domain::entities::{FileContent, FileEntry};
use crate::domain::errors::FileError;
use async_trait::async_trait;
use serde_json::Value;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FileStore: Send + Sync + 'static {
    async fn read(&self, path: &str) -> Result<FileContent, FileError>;
    async fn write(&self, path: &str, content: &str) -> Result<(), FileError>;
    async fn list(&self, path: &str) -> Result<Vec<FileEntry>, FileError>;
    async fn delete(&self, path: &str) -> Result<(), FileError>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DataSource: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn fetch(&self, query: Value) -> Result<Value, FileError>;
}
