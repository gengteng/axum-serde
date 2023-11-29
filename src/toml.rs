//! # TOML

use crate::extractor;
use serde::de::{DeserializeOwned, Error as _};
use serde::Serialize;
use toml_::de::Error as DeError;
use toml_::from_str;
use toml_::to_string;

extractor!(Toml, "application/toml", from_slice, DeError, to_vec);

fn from_slice<T: DeserializeOwned>(s: &[u8]) -> Result<T, DeError> {
    let src = std::str::from_utf8(s).map_err(DeError::custom)?;
    from_str(src)
}

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, toml_::ser::Error> {
    let s = to_string(value)?;
    Ok(s.into_bytes())
}
