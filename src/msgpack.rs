//! # MsgPack

use crate::extractor;
use rmp_serde::decode::Error as DeError;
use rmp_serde::{from_slice, to_vec, to_vec_named};

extractor!(MsgPack, "application/msgpack", from_slice, DeError, to_vec);
extractor!(
    MsgPackRaw,
    "application/msgpack",
    from_slice,
    DeError,
    to_vec_named
);
