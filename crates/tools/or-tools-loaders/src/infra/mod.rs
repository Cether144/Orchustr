#[cfg(feature = "text")]
pub mod text;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "csv")]
pub mod csv_loader;
#[cfg(feature = "pdf")]
pub mod pdf;

pub(crate) mod shared;
