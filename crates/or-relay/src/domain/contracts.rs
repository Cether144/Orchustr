//! Trait boundaries for parallel branch (relay) execution.

#![allow(async_fn_in_trait)]

use crate::domain::errors::RelayError;
use or_core::OrchState;

/// Trait boundary for relay (fan-out / fan-in) executors.
///
/// Any type that can execute a set of parallel branches and merge their
/// results should implement this trait.  The canonical implementation is
/// [`RelayExecutor`](crate::infra::implementations::RelayExecutor).
pub trait RelayExecute<T: OrchState>: Send + Sync {
    /// Execute all branches in the plan concurrently, merging results back
    /// into a single state value.
    async fn execute(
        &self,
        plan: &crate::infra::implementations::RelayPlan<T>,
        initial_state: T,
    ) -> Result<T, RelayError>;
}

/// Trait boundary for relay plan builders.
pub trait RelayBuild<T: OrchState>: Sized {
    /// Add a named branch handler.
    fn add_branch<F, Fut>(self, name: &str, handler: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T, RelayError>> + Send + 'static;
}
