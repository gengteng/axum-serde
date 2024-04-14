//! # CBOR

use crate::extractor;
use serde::de::DeserializeOwned;
use serde::Serialize;

type CborDeError = ciborium::de::Error<std::io::Error>;

extractor!(
    CBOR,
    Cbor,
    "application/cbor",
    from_slice,
    CborDeError,
    to_vec,
    cbor_test
);

fn from_slice<T: DeserializeOwned>(s: &[u8]) -> Result<T, CborDeError> {
    ciborium::de::from_reader(s)
}

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut vec = Vec::with_capacity(128);
    ciborium::into_writer(value, &mut vec)?;
    Ok(vec)
}
