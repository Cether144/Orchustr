//! Trait boundaries for graph-based execution engines.

#![allow(async_fn_in_trait)]

use crate::domain::entities::NodeResult;
use crate::domain::errors::LoomError;
use or_core::OrchState;
use std::future::Future;

/// Trait boundary for graph execution.
///
/// Any type that can execute a state-machine graph over `T` should implement
/// this trait.  The canonical implementation is
/// [`ExecutionGraph`](crate::infra::implementations::ExecutionGraph).
pub trait GraphExecutor<T: OrchState>: Send + Sync {
    /// Run the graph to completion starting from the entry node.
    async fn execute(&self, initial_state: T) -> Result<T, LoomError>;
}

/// Trait boundary for graph builders.
///
/// Defines the fluent API surface that any graph construction strategy must
/// expose.  The canonical implementation is
/// [`GraphBuilder`](crate::infra::implementations::GraphBuilder).
pub trait GraphBuild<T: OrchState>: Sized {
    /// Register a named async node handler.
    fn add_node<F, Fut>(self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<NodeResult<T>, LoomError>> + 'static;

    /// Add a directed edge between two named nodes.
    fn add_edge(self, from: &str, to: &str) -> Self;

    /// Designate the entry node of the graph.
    fn set_entry(self, name: &str) -> Self;

    /// Designate the exit / terminal node of the graph.
    fn set_exit(self, name: &str) -> Self;
}
