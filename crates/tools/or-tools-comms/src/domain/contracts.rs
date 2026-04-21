use super::entities::{Message, SendResult, SocialPost};
use super::errors::CommsError;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait MessageSender: Send + Sync + 'static {
    fn channel(&self) -> &'static str;
    async fn send(&self, msg: Message) -> Result<SendResult, CommsError>;
}

#[async_trait]
pub trait SocialReader: Send + Sync + 'static {
    fn platform(&self) -> &'static str;
    async fn read(&self, query: Value) -> Result<Vec<SocialPost>, CommsError>;
}
