#![allow(dead_code)]
#![allow(unused_variables)]

pub mod wire;

pub use wire::de::Deserializer;
pub use wire::ser::Serializer;
pub use wire::string::ShiftJisString;
pub use wire::FNWireError;

pub fn deserialize<'de, T: serde::Deserialize<'de>>(input: &'de [u8]) -> Result<T, FNWireError> {
    let mut deserializer = Deserializer::new(input);
    T::deserialize(&mut deserializer)
}

pub fn serialize(input: impl serde::Serialize) -> Result<Vec<u8>, FNWireError> {
    let mut buf = vec![];
    let mut serializer = Serializer::new(&mut buf);
    input.serialize(&mut serializer)?;
    Ok(buf)
}
