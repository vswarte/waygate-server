#![allow(dead_code)]
#![allow(unused_variables)]

pub mod wire;
pub mod params;
pub mod payload;
pub mod push;
pub mod request;
pub mod response;

pub use params::*;
pub use payload::*;
pub use request::*;
pub use response::*;
pub use wire::FNWireError;
pub use wire::ser::Serializer;
pub use wire::de::Deserializer;

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
