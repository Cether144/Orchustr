pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::{CommsTool, CommsOrchestrator};
pub use domain::contracts::{MessageSender, SocialReader};
pub use domain::entities::{Channel, Message, SendResult, SocialPost};
pub use domain::errors::CommsError;
