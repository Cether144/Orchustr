use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Channel {
    Sms,
    Telegram,
    Discord,
    WhatsApp,
    Facebook,
    Messenger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub channel: Channel,
    pub to: String,
    pub body: String,
    pub from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub message_id: String,
    pub channel: Channel,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub id: String,
    pub author: String,
    pub body: String,
    pub timestamp: Option<String>,
    pub platform: String,
}
