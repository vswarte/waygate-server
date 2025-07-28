use byteorder::{ReadBytesExt, LE};
use thiserror::Error;

use crate::{eldenring::RequestParams, MessageType};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tried parsing unknown message type. received = {0}.")]
    UnknownMessageType(u8),
    #[error("Io. {0}")]
    Io(#[from] std::io::Error),
    #[error("Action could not be performed on message.")]
    IncorrectMessageType,
    #[error("Could not deserialize message.")]
    Wire(#[from] wire::FNWireError),
}

/// Buffer wrapper for reading RPC messages.
pub struct MessageReader<'a>(&'a [u8]);

impl<'a> MessageReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    /// Read message type from the contained buffer.
    pub fn message_type(&self) -> Result<MessageType, Error> {
        let mut reader = self.0;
        reader.read_u8()?.try_into()
    }

    /// Read sequence # of the contained message, returns None if the contained message is malformed
    /// or if the message is not a request or a response.
    pub fn sequence(&self) -> Result<Option<u32>, Error> {
        let mut reader = self.0;

        Ok(match reader.read_u8()?.try_into()? {
            MessageType::Request | MessageType::Response => Some(reader.read_u32::<LE>()?),
            _ => None,
        })
    }

    /// Read variant of the contained message, returns None if the contained message is malformed
    /// or if the message is not a request or a response.
    pub fn peek_variant(&self) -> Result<Option<u32>, Error> {
        let mut reader = self.0;

        Ok(match reader.read_u8()?.try_into()? {
            MessageType::Request | MessageType::Response => {
                // Skip the sequence number
                reader.read_u32::<LE>()?;
                // Read the variant
                Some(reader.read_u32::<LE>()?)
            }
            _ => None,
        })
    }

    /// Retrieve deserialized contents for a request.
    pub fn deserialize_request(&self) -> Result<RequestParams, Error> {
        let mut reader = self.0;

        if !matches!(reader.read_u8()?.try_into()?, MessageType::Request) {
            return Err(Error::IncorrectMessageType);
        }

        let _sequence = reader.read_u32::<LE>()?;

        Ok(wire::deserialize::<RequestParams>(reader)?)
    }
}

pub enum MessageContents {}

impl TryFrom<u8> for MessageType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Error> {
        Ok(match value {
            0x4 => Self::Request,
            0x5 => Self::Response,
            0x6 => Self::Push,
            0x7 => Self::Heartbeat,
            _ => return Err(Error::UnknownMessageType(value)),
        })
    }
}
