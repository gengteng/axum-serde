//! # YAML

use crate::extractor;
use serde::Serialize;
use serde_yaml::from_slice;
use serde_yaml::to_writer;
use serde_yaml::Error;

extractor!(YAML, Yaml, application, yaml, from_slice, Error, to_vec);

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::with_capacity(128);
    to_writer(&mut buffer, value)?;
    Ok(buffer.to_vec())
}
