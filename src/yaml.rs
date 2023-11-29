//! # YAML

use crate::extractor;
use serde::Serialize;
use serde_yaml::from_slice;
use serde_yaml::to_string;
use serde_yaml::Error;

extractor!(Yaml, "application/yaml", from_slice, Error, to_vec);

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let s = to_string(value)?;
    Ok(s.into_bytes())
}
