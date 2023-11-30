//! # MessagePack

use crate::extractor;
use rmp_serde::decode::Error;
use rmp_serde::{from_slice, to_vec, to_vec_named};

extractor!(
    MessagePack,
    MsgPack,
    application,
    msgpack,
    from_slice,
    Error,
    to_vec_named,
    msgpack_test
);

extractor!(
    MessagePack,
    MsgPackRaw,
    application,
    msgpack,
    from_slice,
    Error,
    to_vec,
    msgpack_raw_test
);
