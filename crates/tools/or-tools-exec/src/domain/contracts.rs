use crate::domain::entities::{ExecRequest, ExecResult};
use crate::domain::errors::ExecError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CodeExecutor: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn supports(&self, lang: crate::domain::entities::Language) -> bool;
    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError>;
}
