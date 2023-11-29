# axum-serde

## 📑 Overview

**axum-serde** is a library that provides multiple serde-based extractors and responders for the Axum web framework.

## 🚀 Basic usage

## 🗂️ Extractors

| Extractor                      | Feature | Backend                                           | Tests |
|--------------------------------|---------|---------------------------------------------------|-------|
| `Yaml<T>`                      | yaml    | [serde_yaml](https://crates.io/crates/serde_yaml) | ❌     | 
| `MsgPack<T>` / `MsgPackRaw<T>` | msgpack | [rmp-serde](https://crates.io/crates/rmp-serde)   | ❌     | 
| `Toml<T>`                      | toml    | [toml](https://crates.io/crates/toml)             | ❌     | 

## 📜 License

This project is licensed under the MIT License.

## 📚 References

* [axum](https://crates.io/crates/axum)