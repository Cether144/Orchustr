use super::entities::{CalendarEvent, Email, Issue, Page};
use super::errors::ProductivityError;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait EmailClient: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn list(&self, query: Value) -> Result<Vec<Email>, ProductivityError>;
    async fn send_email(&self, email: Email) -> Result<String, ProductivityError>;
}

#[async_trait]
pub trait CalendarClient: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn list_events(&self, query: Value) -> Result<Vec<CalendarEvent>, ProductivityError>;
    async fn create_event(&self, event: CalendarEvent) -> Result<String, ProductivityError>;
}

#[async_trait]
pub trait ProjectTracker: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn list_issues(&self, query: Value) -> Result<Vec<Issue>, ProductivityError>;
    async fn create_issue(&self, issue: Issue) -> Result<String, ProductivityError>;
}

#[async_trait]
pub trait KnowledgeBase: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn search(&self, query: Value) -> Result<Vec<Page>, ProductivityError>;
    async fn create_page(&self, page: Page) -> Result<String, ProductivityError>;
}

#[async_trait]
pub trait TeamMessenger: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn post(&self, channel: &str, text: &str) -> Result<String, ProductivityError>;
    async fn search_messages(&self, query: Value) -> Result<Vec<Page>, ProductivityError>;
}
