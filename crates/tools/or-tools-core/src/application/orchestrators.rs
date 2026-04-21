use crate::domain::contracts::ToolRegistry;
use crate::domain::entities::{ToolInput, ToolOutput};
use crate::domain::errors::ToolError;
use std::sync::Arc;
use std::time::Instant;

/// Orchestrates dispatch of a [`ToolInput`] to the matching [`Tool`] via a
/// [`ToolRegistry`]. Adds timing + structured logging.
#[derive(Clone)]
pub struct ToolDispatcher {
    registry: Arc<dyn ToolRegistry>,
}

impl ToolDispatcher {
    #[must_use]
    pub fn new(registry: Arc<dyn ToolRegistry>) -> Self {
        Self { registry }
    }

    pub async fn dispatch(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let span = tracing::info_span!(
            "tools.dispatch",
            otel.name = "tools.dispatch",
            tool = %input.tool,
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let start = Instant::now();

        let tool = self.registry.get(&input.tool).await?;
        let result = tool.invoke(input).await;

        let elapsed = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        match result {
            Ok(mut out) => {
                out.duration_ms = elapsed;
                span.record("status", "success");
                Ok(out)
            }
            Err(err) => {
                span.record("status", "failure");
                Err(err)
            }
        }
    }
}
