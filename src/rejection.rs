//! # Rejection
//!

use axum::http::StatusCode;
use axum_core::extract::rejection::BytesRejection;
use axum_core::response::{IntoResponse, Response};
use std::fmt::Display;

/// The `Rejection` enum represents the different types of rejections that can occur when handling
/// requests with unsupported media types or invalid content formats.
///
/// # Enums
///
/// - `UnsupportedMediaType`: Represents a rejection when the media type of the request is not supported.
/// - `InvalidContentFormat(E)`: Represents a rejection when the content format of the request is invalid,
///   and includes an associated error of type `E`.
///
#[derive(Debug, thiserror::Error)]
pub enum Rejection<E> {
    /// The `UnsupportedMediaType` variant represents an HTTP 415 Unsupported Media Type error.
    /// This error occurs when the server cannot handle the media type specified in the request.
    ///
    /// * [HTTP 415 Unsupported Media Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/415)
    #[error("Unsupported Media Type. The server expects requests with 'Content-Type: {0}'.")]
    UnsupportedMediaType(&'static str),
    /// Failed to buffer body
    ///
    /// * [HTTP 400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
    #[error(transparent)]
    BytesRejection(#[from] BytesRejection),
    /// The `InvalidContentFormat` variant represents an HTTP 422 Unprocessable Content error with an associated error message.
    /// This error occurs when the server cannot handle the content in the request.
    ///
    /// * [HTTP 422 Unprocessable Content](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)
    ///
    #[error("Invalid content format: {0}")]
    InvalidContentFormat(E),
}

impl<E: Display> IntoResponse for Rejection<E> {
    fn into_response(self) -> Response {
        match self {
            Rejection::UnsupportedMediaType(expected) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Unsupported Media Type. The server expects requests with 'Content-Type: {expected}'."),
            )
                .into_response(),
            Rejection::BytesRejection(br) => br.into_response(),
            Rejection::InvalidContentFormat(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response()
            }
        }
    }
}
