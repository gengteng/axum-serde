# axum-serde

[![crates.io](https://img.shields.io/crates/v/axum-serde)](https://crates.io/crates/axum-serde)
[![crates.io download](https://img.shields.io/crates/d/axum-serde)](https://crates.io/crates/axum-serde)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue)](https://github.com/gengteng/axum-serde/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/gengteng/axum-serde/status.svg)](https://deps.rs/repo/github/gengteng/axum-serde)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/gengteng/axum-serde/.github/workflows/main.yml?branch=main)](https://github.com/gengteng/axum-serde/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/gengteng/axum-serde/badge.svg?branch=main)](https://coveralls.io/github/gengteng/axum-serde?branch=main)

## 📑 Overview

**axum-serde** is a library that provides multiple serde-based extractors / responses for the Axum web framework. It
also offers a macro to easily customize extractors and responses without writing much boilerplate code.

If you were using crates like **axum-yaml**, **axum-msgpack** etc. in axum 0.6 and wish to upgrade to axum 0.7,
**axum-serde** can be used as a replacement to simplify the migration, without having to modify existing code too much.

## 🚀 Basic usage

* Install

```shell
cargo add axum-serde --features yaml,sonic
# Enable features as you need
```

* Example

```rust,ignore
use axum::routing::post;
use axum::Router;
use axum_serde::{Sonic, Yaml};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub a: i32,
    pub b: String,
}

pub async fn yaml_to_json(Yaml(data): Yaml<Data>) -> Sonic<Data> {
    Sonic(data)
}

pub async fn json_to_yaml(Sonic(data): Sonic<Data>) -> Yaml<Data> {
    Yaml(data)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/y2j", post(yaml_to_json))
        .route("/j2y", post(json_to_yaml));
    let listener = TcpListener::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080))).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
```

* Test

```shell
curl -X POST http://localhost:8080/y2j -H "Content-Type: application/yaml" -d $'a: 42\nb: Hello'
curl -X POST http://localhost:8080/j2y -H "Content-Type: application/json" -d '{"a": 42, "b": "Hello, world!"}'
```

## 🗂️ Extractors / responses

| Extractor                      | Feature | Backend                                                   |
|--------------------------------|---------|-----------------------------------------------------------|
| `Yaml<T>`                      | yaml    | [serde_yaml](https://crates.io/crates/serde_yaml) v0.9.33 |
| `MsgPack<T>` / `MsgPackRaw<T>` | msgpack | [rmp-serde](https://crates.io/crates/rmp-serde) v1.3.0    |
| `Toml<T>`                      | toml    | [toml](https://crates.io/crates/toml) v0.8.14             |
| `Xml<T>`                       | xml     | [quick-xml](https://crates.io/crates/quick-xml) v0.36.1   |
| `Sonic<T>`                     | sonic   | [sonic-rs](https://crates.io/crates/sonic-rs) v0.3.7      |
| `Cbor<T>`                      | cbor    | [ciborium](https://crates.io/crates/ciborium) v0.2.2      |

## 🎁 Custom extractor / response

Use the `extractor` macro to create custom extractors with minimal boilerplate:

* Example

```rust
use axum_serde::{
    extractor,
    macros::{DeserializeOwned, Serialize},
};

extractor!(
    MyFormat,                   // The name of the data format.
    MyFmt,                      // The actual type name of the HTTP extractor/response.
    "application/myfmt",        // The Content-Type that this extractor supports.
    from_slice,                 // A function identifier for deserializing data from the HTTP request body.
    String,                     // The type of error that can occur when deserializing from the request body.
    to_vec,                     // A function identifier for serializing the HTTP response body to bytes.
    myfmt                       // The test module name.
);

fn from_slice<T: DeserializeOwned>(_bytes: &[u8]) -> Result<T, String> {
    todo!()
}

fn to_vec<T: Serialize>(_value: &T) -> Result<Vec<u8>, String> {
    todo!()
}
```

* Test

More `dev-dependencies` are required to run the tests:

```shell
# Add dev-dependencies for tests
cargo add axum-test --dev
cargo add serde --features derive --dev
cargo add tokio --features macros --dev

# Run the generated tests
cargo test myfmt
```

## 📜 License

This project is licensed under the MIT License.

## 📚 References

* [axum](https://crates.io/crates/axum)
* [serde](https://crates.io/crates/serde)