//! # Rejection
//!

use axum::http::StatusCode;
use axum_core::extract::rejection::BytesRejection;
use axum_core::response::{IntoResponse, Response};
use std::fmt::Display;

/// `Rejection` is an enumeration which describes different types of rejection
/// responses from handling HTTP requests. It is designed to provide informative,
/// readable error responses which can be converted into HTTP responses.
///
/// This enumeration includes the following variants:
///
/// * `UnsupportedMediaType(&'static str)`: This variant represents an `HTTP 415 Unsupported Media Type` error.
///   It occurs when the server cannot handle the media type specified in the request. The associated string
///   represents the expected media type.
///
/// * `BytesRejection(#[from] BytesRejection)`: This variant encapsulates errors when trying to read the body of
///   the request. It wraps the `BytesRejection` error type, effectively providing a conversion from `BytesRejection`
///   errors to a `Rejection` error.
///
/// * `InvalidContentFormat(E)`: This variant represents an `HTTP 422 Unprocessable Entity` error. It is used when
///   the server cannot process the content format of the request. The associated generic type `E` allows flexibility
///   in what exact error information this variant carries.
///
/// Each variant includes a detailed error message suitable for converting into an HTTP response.
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
                (StatusCode::UNPROCESSABLE_ENTITY, format!("Invalid content format: {}", e)).into_response()
            }
        }
    }
}
