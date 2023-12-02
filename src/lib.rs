#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

pub mod macros;
#[cfg(feature = "msgpack")]
pub mod msgpack;
pub mod rejection;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "xml")]
pub mod xml;
#[cfg(feature = "yaml")]
pub mod yaml;

use axum::http::{header, HeaderMap};
use mime::Mime;

#[cfg(feature = "msgpack")]
pub use msgpack::{MsgPack, MsgPackRaw};
pub use rejection::Rejection;
#[cfg(feature = "toml")]
pub use toml::Toml;
#[cfg(feature = "xml")]
pub use xml::Xml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;

/// Checks if the content type in the given headers matches the expected content type.
///
/// # Arguments
///
/// * `headers` - A reference to the `HeaderMap` containing the headers.
/// * `expected_content_type` - A reference to the `mime::Mime` representing the expected content type.
///
/// # Returns
///
/// Returns `true` if the content type in the headers matches the expected content type, otherwise `false`.
pub fn check_content_type(headers: &HeaderMap, expected_content_type: &str) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    <Mime as PartialEq<&str>>::eq(&mime, &expected_content_type)
}
