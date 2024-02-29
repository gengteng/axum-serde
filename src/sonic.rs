//! # JSON (Sonic)
//!
use serde::de::{DeserializeOwned, Error as _};
use serde::Serialize;
use sonic_rs::Error;
use crate::extractor;

extractor!(JSON,
    Sonic,
    "application/json",
    from_slice,
    Error,
    to_vec,
    sonic_test
);

fn from_slice<T: DeserializeOwned>(s: &[u8]) -> Result<T, Error> {
    let src = std::str::from_utf8(s).map_err(Error::custom)?;
    sonic_rs::from_str(src)
}

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let s = sonic_rs::to_string(value)?;
    Ok(s.into_bytes())
}