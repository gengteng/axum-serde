//! # XML

use crate::extractor;
use quick_xml::de::from_reader;
use quick_xml::se::to_writer;
use quick_xml::DeError;
use serde::de::DeserializeOwned;
use serde::Serialize;

fn from_slice<T: DeserializeOwned>(s: &[u8]) -> Result<T, DeError> {
    from_reader(s)
}

fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, DeError> {
    let mut buffer = bytes::BytesMut::with_capacity(128);
    to_writer(&mut buffer, value)?;
    Ok(buffer.to_vec())
}

extractor!(
    XML,
    Xml,
    application,
    xml,
    from_slice,
    DeError,
    to_vec,
    xml_test
);
