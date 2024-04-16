pub mod ser;
pub mod de;

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FNWireError {
    #[error("Unspecified error: {0}")]
    Other(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("UTF8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Sequence size overflow: {0}")]
    SizeOverflow(usize),
    #[error("Unsupported type: {0}")]
    UnsupportedType(&'static str),
    #[error("Sequence of unknown size")]
    SizelessSeq,
    #[error("EOF reached prematurely")]
    Eof,
    #[error("Variant index ({0}) is not valid for this type")]
    BadVariant(u32),
    #[error("Value {0} is not valid type \"{1}\"")]
    BadValue(u64, &'static str),
    #[error("Format is not self-describing")]
    NotSelfDescribing,
}

#[cfg(test)]
mod tests {
    use crate::{Serializer, Deserializer};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Eq)]
    struct UsedItem {
        pub item_id: u32,
        pub times_used: u32,
        pub unk_08: u32,
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    struct Location {
        pub map_id: u32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    struct RequestUseItemLog {
        pub used_items: Vec<UsedItem>,
        pub use_location: Location,
    }

    #[test]
    fn request_use_item_serialization() {
        let mut buf: Vec<u8> = Vec::new();

        let msg = RequestUseItemLog {
            used_items: vec![
                UsedItem {
                    item_id: 0x69,
                    times_used: 1,
                    unk_08: 0,
                },
                UsedItem {
                    item_id: 0x420,
                    times_used: 5,
                    unk_08: 1,
                },
                UsedItem {
                    item_id: 0x42069,
                    times_used: 10,
                    unk_08: 2,
                },
            ],
            use_location: Location {
                map_id: 0xDEADBEEF,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        msg.serialize(&mut Serializer::new(&mut buf)).unwrap();

        assert!(
            buf == [
                0x03, 0x00, 0x00, 0x00, 0x69, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x20, 0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x69, 0x20, 0x04, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xEF, 0xBE,
                0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );

        let msg_deser =
            RequestUseItemLog::deserialize(&mut Deserializer::new(buf.as_slice())).unwrap();
        assert!(msg_deser == msg)
    }
}
