use crate::domain::entities::{FetchRequest, FetchResponse, ScrapedPage};
use crate::domain::errors::WebError;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait WebBrowser: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Scraper: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn scrape(&self, url: &str) -> Result<ScrapedPage, WebError>;
}
