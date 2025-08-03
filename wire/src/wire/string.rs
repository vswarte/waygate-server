use std::ops::{Deref, DerefMut};

use encoding_rs::SHIFT_JIS;
use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct ShiftJisDecodeError {
    bytes: Vec<u8>,
}

impl std::fmt::Display for ShiftJisDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to decode Shift JIS bytes: {:?}", self.bytes)
    }
}

#[derive(Debug)]
pub enum ShiftJisError {
    EncodingError(String),
    DecodingError(ShiftJisDecodeError),
}

impl std::fmt::Display for ShiftJisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShiftJisError::EncodingError(err) => write!(f, "Shift JIS encoding error: {err}"),
            ShiftJisError::DecodingError(err) => write!(f, "Shift JIS decoding error: {err}"),
        }
    }
}

impl std::error::Error for ShiftJisError {}

struct ShiftJisStringVisitor;

impl<'de> de::Visitor<'de> for ShiftJisStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array representing a shift-jis string")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let (decoded, _, had_errors) = SHIFT_JIS.decode(v);
        if had_errors {
            return Err(E::custom(ShiftJisError::DecodingError(
                ShiftJisDecodeError { bytes: v.to_vec() },
            )));
        }
        Ok(decoded.into_owned())
    }
}

/// A wrapper around `String` that serializes and deserializes as a Shift JIS
/// encoded, length-prefixed byte array.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShiftJisString(pub String);

impl From<String> for ShiftJisString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<ShiftJisString> for String {
    fn from(s: ShiftJisString) -> Self {
        s.0
    }
}

impl Deref for ShiftJisString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<String> for ShiftJisString {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for ShiftJisString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl DerefMut for ShiftJisString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for ShiftJisString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (encoded, _, had_errors) = SHIFT_JIS.encode(&self.0);
        if had_errors {
            return Err(ser::Error::custom(ShiftJisError::EncodingError(
                "Failed to encode string as Shift-JIS".to_string(),
            )));
        }
        serializer.serialize_bytes(&encoded)
    }
}

impl<'de> Deserialize<'de> for ShiftJisString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_bytes(ShiftJisStringVisitor)
            .map(ShiftJisString)
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize;

    use super::*;

    #[test]
    fn test_shift_jis_string_serialization() {
        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            id: u32,
            shift_jis: ShiftJisString,
            utf8: String,
        }

        static EXPECTED_BYTES: &[u8] = &[
            0x01, 0x00, 0x00, 0x00, // id
            0x0a, 0x00, 0x00, 0x00, // length of string
            0x82, 0xb1, 0x82, 0xf1, 0x82, 0xc9, 0x82, 0xbf, 0x82,
            0xcd, // "こんにちは" in Shift JIS
            0x0f, 0x00, 0x00, 0x00, // length of UTF-8 string
            0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81,
            0xaf, // "こんにちは" in UTF-8
        ];

        let my_struct = MyStruct {
            id: 1,
            shift_jis: ShiftJisString(String::from("こんにちは")),
            utf8: String::from("こんにちは"),
        };

        let serialized = serialize(my_struct);
        assert!(serialized.is_ok());
        let serialized_bytes = serialized.unwrap();
        assert_eq!(serialized_bytes, EXPECTED_BYTES);
    }
}
