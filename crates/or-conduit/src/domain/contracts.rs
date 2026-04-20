#![allow(async_fn_in_trait)]

use crate::domain::entities::{CompletionMessage, CompletionResponse, MessageRole};
use crate::domain::errors::ConduitError;
use futures::{Stream, stream};
use std::pin::Pin;

pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, ConduitError>> + Send>>;

#[cfg_attr(test, mockall::automock)]
pub trait ConduitProvider: Send + Sync + 'static {
    async fn complete_messages(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<CompletionResponse, ConduitError>;

    async fn complete_text(&self, prompt: &str) -> Result<CompletionResponse, ConduitError> {
        self.complete_messages(vec![CompletionMessage::single_text(
            MessageRole::User,
            prompt,
        )])
        .await
    }

    /// Streams completion text chunk-by-chunk.
    ///
    /// # Default fallback
    ///
    /// The default implementation calls [`complete_messages`] and emits the
    /// entire response as a **single chunk**. This is **not** true streaming
    /// — providers **must** override this method to deliver incremental
    /// Server-Sent-Event (SSE) chunks for a real streaming experience.
    async fn stream_text(
        &self,
        messages: Vec<CompletionMessage>,
    ) -> Result<TextStream, ConduitError> {
        tracing::warn!(
            "ConduitProvider::stream_text is using the non-streaming fallback. \
             Override stream_text in your provider for true token-level streaming."
        );
        let response = self.complete_messages(messages).await?;
        Ok(Box::pin(stream::iter(std::iter::once(Ok(response.text)))))
    }
}
