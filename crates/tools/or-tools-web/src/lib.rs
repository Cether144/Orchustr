//! Web-browsing & scraping tool implementations.

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::WebOrchestrator;
pub use domain::contracts::{Scraper, WebBrowser};
pub use domain::entities::{FetchRequest, FetchResponse, HttpMethod, ScrapedPage};
pub use domain::errors::WebError;
