//! Trait boundaries for sequential pipeline execution.

#![allow(async_fn_in_trait)]

use crate::domain::errors::PipelineError;
use or_core::OrchState;

/// Trait boundary for sequential pipeline executors.
///
/// Any type that can execute a linear chain of nodes should implement this
/// trait.  The canonical implementation is
/// [`Pipeline`](crate::infra::implementations::Pipeline).
pub trait PipelineExecute<T: OrchState>: Send + Sync {
    /// Run every node in order, threading state through each.
    async fn execute(&self, initial_state: T) -> Result<T, PipelineError>;
}

/// Trait boundary for pipeline builders.
pub trait PipelineBuild<T: OrchState>: Sized {
    /// Append a named async node handler to the pipeline.
    fn add_node<F, Fut>(self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T, PipelineError>> + Send + 'static;
}
