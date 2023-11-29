#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

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
pub use msgpack::*;
pub use rejection::*;
#[cfg(feature = "toml")]
pub use toml::*;
#[cfg(feature = "xml")]
pub use xml::*;
#[cfg(feature = "yaml")]
pub use yaml::*;

/// This macro is designed to create an extractor type.
/// It uses `serde` for extracting data from requests and serializing data into response body.
///
#[macro_export]
macro_rules! extractor {
    (
        $ext:tt,
        $type_:tt,
        $subtype:tt,
        $de:ident,
        $de_err:ident,
        $ser:ident
    ) => {
        #[doc = stringify!($ext)]
        #[doc = " Extractor / Response."]
        #[doc = ""]
        #[doc = "When used as an extractor, it can deserialize request bodies into some type that"]
        #[doc = "implements [`serde::Deserialize`]. The request will be rejected (and a [`crate::Rejection`] will"]
        #[doc = "be returned) if:"]
        #[doc = "- The request doesn't have a `Content-Type:"]
        #[doc = concat!(stringify!($type_), "/", stringify!($subtype))]
        #[doc = "` (or similar) header."]
        #[doc = "- The body doesn't contain syntactically valid "]
        #[doc = stringify!($ext)]
        #[doc = "."]
        #[doc = "- The body contains syntactically valid "]
        #[doc = stringify!($ext)]
        #[doc = " but it couldn't be deserialized into the target"]
        #[doc = "type."]
        #[doc = "- Buffering the request body fails."]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $ext<T>(pub T);

        impl<T> From<T> for $ext<T> {
            fn from(inner: T) -> Self {
                Self(inner)
            }
        }

        impl<T> std::ops::Deref for $ext<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $ext<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> $ext<T> {
            #[doc = "Consumes the extractor and returns the inner data."]
            pub fn into_inner(self) -> T {
                self.0
            }

            #[doc = "Content type of "]
            #[doc = stringify!($ext)]
            #[doc = " format."]
            pub const CONTENT_TYPE: &'static str = concat!(stringify!($type_), "/", stringify!($subtype));
        }

        #[async_trait::async_trait]
        impl<T, S> axum::extract::FromRequest<S> for $ext<T>
        where
            T: serde::de::DeserializeOwned,
            S: Send + Sync,
        {
            type Rejection = $crate::Rejection<$de_err>;

            async fn from_request(
                req: axum::extract::Request,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                if $crate::check_content_type(req.headers(), Self::CONTENT_TYPE) {
                    let src = bytes::Bytes::from_request(req, state).await?;
                    let inner = $de(&src).map_err($crate::Rejection::InvalidContentFormat)?;
                    Ok($ext(inner))
                } else {
                    Err($crate::Rejection::UnsupportedMediaType(Self::CONTENT_TYPE))
                }
            }
        }

        impl<T> axum::response::IntoResponse for $ext<T>
        where
            T: serde::Serialize,
        {
            fn into_response(self) -> axum::response::Response {
                match $ser(&self.0) {
                    Ok(vec) => (
                        [(
                            axum::http::header::CONTENT_TYPE,
                            axum::http::HeaderValue::from_static(Self::CONTENT_TYPE),
                        )],
                        vec,
                    )
                        .into_response(),
                    Err(err) => (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        [(
                            axum::http::header::CONTENT_TYPE,
                            axum::http::HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                        )],
                        err.to_string(),
                    )
                        .into_response(),
                }
            }
        }
    };
}

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
