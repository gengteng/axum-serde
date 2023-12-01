//! Extractor macro
//!

/// This macro is designed to create an extractor type.
/// It uses `serde` for extracting data from requests and serializing data into response body.
///
/// # Arguments
///
/// * `$name` - The name of the HTTP extractor/response.
/// * `$ext` - The actual type name of the HTTP extractor/response.
/// * `$type_` - The main type of the Content-Type that this extractor supports.
/// * `$subtype` - The subtype of the Content-Type that this extractor supports.
/// * `$de` - A function identifier for deserializing data from the HTTP request body.
/// * `$de_err` - The type of error that can occur when deserializing from the request body.
/// * `$ser` - A function identifier for serializing the HTTP response body to bytes.
/// * `$test` - The test module name.
///
#[macro_export]
macro_rules! extractor {
    (
        $name:tt,
        $ext:tt,
        $type_:tt,
        $subtype:tt,
        $de:ident,
        $de_err:ident,
        $ser:ident,
        $test:ident
    ) => {
        #[doc = stringify!($name)]
        #[doc = " Extractor / Response."]
        #[doc = ""]
        #[doc = "When used as an extractor, it can deserialize request bodies into some type that"]
        #[doc = "implements [`serde::Deserialize`]. The request will be rejected (and a [`crate::Rejection`] will"]
        #[doc = "be returned) if:"]
        #[doc = "- The request doesn't have a `Content-Type:"]
        #[doc = concat!(stringify!($type_), "/", stringify!($subtype))]
        #[doc = "` (or similar) header."]
        #[doc = "- The body doesn't contain syntactically valid "]
        #[doc = stringify!($name)]
        #[doc = "."]
        #[doc = "- The body contains syntactically valid "]
        #[doc = stringify!($name)]
        #[doc = " but it couldn't be deserialized into the target"]
        #[doc = "type."]
        #[doc = "- Buffering the request body fails."]
        #[doc = ""]
        #[doc = "⚠️ Since parsing "]
        #[doc = stringify!($name)]
        #[doc = " requires consuming the request body, the "]
        #[doc = stringify!($ext)]
        #[doc = " extractor must be last if there are multiple extractors in a handler."]
        #[doc = ""]
        #[doc = "See [`crate::Rejection`] for more details."]
        #[doc = "# Extractor example"]
        #[doc = " ```rust,ignore"]
        #[doc = " use axum::{"]
        #[doc = "     routing::post,"]
        #[doc = "     Router,"]
        #[doc = " };"]
        #[doc = concat!(" use axum_serde::", stringify!($ext), ";")]
        #[doc = " use serde::Deserialize;"]
        #[doc = ""]
        #[doc = " #[derive(Deserialize)]"]
        #[doc = " struct CreateUser {"]
        #[doc = "     email: String,"]
        #[doc = "     password: String,"]
        #[doc = " }"]
        #[doc = ""]
        #[doc = concat!(" async fn create_user(", stringify!($ext), "(payload): ", stringify!($ext), "<CreateUser>) {")]
        #[doc = "     // payload is a `CreateUser`"]
        #[doc = " }"]
        #[doc = ""]
        #[doc = " let app = Router::new().route(\"/users\", post(create_user));"]
        #[doc = " # let _: Router = app;"]
        #[doc = " ```"]
        #[doc = " When used as a response, it can serialize any type that implements [`serde::Serialize`] to"]
        #[doc = " `"]
        #[doc = stringify!($ext)]
        #[doc = "`, and will automatically set `Content-Type:"]
        #[doc = concat!(stringify!($type_), "/", stringify!($subtype))]
        #[doc = "` header."]
        #[doc = ""]
        #[doc = " # Response example"]
        #[doc = ""]
        #[doc = " ```rust,ignore"]
        #[doc = " use axum::{"]
        #[doc = "     extract::Path,"]
        #[doc = "     routing::get,"]
        #[doc = "     Router,"]
        #[doc = " };"]
        #[doc = concat!(" use axum_serde::", stringify!($ext), ";")]
        #[doc = " use serde::Serialize;"]
        #[doc = " use uuid::Uuid;"]
        #[doc = ""]
        #[doc = " #[derive(Serialize)]"]
        #[doc = " struct User {"]
        #[doc = "     id: Uuid,"]
        #[doc = "     username: String,"]
        #[doc = " }"]
        #[doc = ""]
        #[doc = concat!(" async fn get_user(Path(user_id) : Path<Uuid>) -> ", stringify!($ext), "<User> {")]
        #[doc = "     let user = find_user(user_id).await;"]
        #[doc = concat!("     ", stringify!($ext), "(user)")]
        #[doc = " }"]
        #[doc = ""]
        #[doc = " async fn find_user(user_id: Uuid) -> User {"]
        #[doc = "     // ..."]
        #[doc = "     # unimplemented!()"]
        #[doc = " }"]
        #[doc = ""]
        #[doc = " let app = Router::new().route(\"/users/:id\", get(get_user));"]
        #[doc = " # let _: Router = app;"]
        #[doc = " ```"]
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
            #[doc = "Consumes the `"]
            #[doc = stringify!($ext)]
            #[doc = "` extractor and returns the inner data."]
            pub fn into_inner(self) -> T {
                self.0
            }

            #[doc = "Content type of "]
            #[doc = stringify!($name)]
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

        #[cfg(test)]
        mod $test {
            use super::*;
            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize, Default)]
            struct Value {
                v0: String,
                v1: i32,
            }

            const TEST_ROUTE: &'static str = "/value";
            const EXT_CONTENT_TYPE: &'static str = concat!(stringify!($type_), "/", stringify!($subtype));

            #[tokio::test]
            async fn extractor() {
                use axum::Router;
                use axum::routing::post;
                use axum_test::TestServer;
                use axum_test::http::HeaderValue;

                async fn handler($ext(_user): $ext<Value>) {
                }

                let my_app = Router::new()
                    .route(TEST_ROUTE, post(handler));

                let server = TestServer::new(my_app).expect("Failed to create test server");

                let value = Value::default();
                let bytes = bytes::Bytes::from($ser(&value).expect("Failed to serialize value"));

                let response = server.post(TEST_ROUTE)
                    .bytes(bytes.clone())
                    .add_header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static(EXT_CONTENT_TYPE))
                    .await;

                assert_eq!(response.status_code(), axum_test::http::StatusCode::OK);

                let response = server.post(TEST_ROUTE)
                    .bytes(bytes.clone())
                    .await;
                assert_eq!(response.status_code(), axum_test::http::StatusCode::UNSUPPORTED_MEDIA_TYPE);

                let response = server.post(TEST_ROUTE)
                    .bytes(bytes)
                    .add_header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("invalid/type"))
                    .await;
                assert_eq!(response.status_code(), axum_test::http::StatusCode::UNSUPPORTED_MEDIA_TYPE);

                let response = server.post(TEST_ROUTE)
                    .bytes(bytes::Bytes::from_static(b"invalid data"))
                    .add_header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static(EXT_CONTENT_TYPE))
                    .await;
                assert_eq!(response.status_code(), axum_test::http::StatusCode::UNPROCESSABLE_ENTITY);
            }

            #[tokio::test]
            async fn response() {
                use axum::Router;
                use axum::routing::get;
                use axum_test::TestServer;

                async fn handler() -> $ext<Value> {
                    $ext(Value::default())
                }

                let my_app = Router::new()
                    .route(TEST_ROUTE, get(handler));

                let server = TestServer::new(my_app).expect("Failed to create test server");

                let response = server.get(TEST_ROUTE).await;
                assert!($crate::check_content_type(response.headers(), EXT_CONTENT_TYPE));
                let body = response.as_bytes();
                let _value: Value = $de(&body).expect("Failed to deserialize");
            }
        }
    };
}
