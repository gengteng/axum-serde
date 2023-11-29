//! # MsgPack

use crate::extractor;
use rmp_serde::decode::Error;
use rmp_serde::{from_slice, to_vec, to_vec_named};

extractor!(
    MsgPack,
    "application/msgpack",
    from_slice,
    Error,
    to_vec_named
);
extractor!(MsgPackRaw, "application/msgpack", from_slice, Error, to_vec);
