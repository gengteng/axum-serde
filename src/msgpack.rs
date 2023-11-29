//! # MsgPack

use crate::extractor;
use rmp_serde::decode::Error as DeError;
use rmp_serde::from_slice;
use rmp_serde::to_vec;

extractor!(MsgPack, "application/msgpack", from_slice, DeError, to_vec);
