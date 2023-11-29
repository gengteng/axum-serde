# axum-serde

[![crates.io](https://img.shields.io/crates/v/axum-serde)](https://crates.io/crates/axum-serde)
[![crates.io download](https://img.shields.io/crates/d/axum-serde)](https://crates.io/crates/axum-serde)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue)](https://github.com/gengteng/axum-serde/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/gengteng/axum-serde/status.svg)](https://deps.rs/repo/github/gengteng/axum-serde)

## 📑 Overview

**axum-serde** is a library that provides multiple serde-based extractors and responders for the Axum web framework.

## 🚀 Basic usage

* Install

```shell
cargo add axum-serde --features yaml
# Enable features as you need
```

* Example

```rust,ignore
use axum::routing::post;
use axum::Router;
use axum_serde::Yaml;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub v0: usize,
    pub v1: usize,
}

pub async fn handler(Yaml(_data): Yaml<Data>) {
    // use _data
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/data", post(handler));
    let listener = TcpListener::bind(&SocketAddr::from(([0u8, 0, 0, 0], 0u16))).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
```

## 🗂️ Extractors

| Extractor                      | Feature | Backend                                                   |
|--------------------------------|---------|-----------------------------------------------------------|
| `Yaml<T>`                      | yaml    | [serde_yaml](https://crates.io/crates/serde_yaml) v0.9.27 |
| `MsgPack<T>` / `MsgPackRaw<T>` | msgpack | [rmp-serde](https://crates.io/crates/rmp-serde) v1.1.2    |
| `Toml<T>`                      | toml    | [toml](https://crates.io/crates/toml) v0.8.8              |
| `Xml<T>`                       | xml     | [quick-xml](https://crates.io/crates/quick-xml) v0.31.0   |

## 📜 License

This project is licensed under the MIT License.

## 📚 References

* [axum](https://crates.io/crates/axum)
* [serde](https://crates.io/crates/serde)