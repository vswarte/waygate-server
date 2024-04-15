pub(crate) mod handler;
pub(crate) mod session;
pub(crate) mod player;
pub(crate) mod announcement;
pub(crate) mod ghostdata;
pub(crate) mod bloodstain;
pub(crate) mod bloodmessage;
pub(crate) mod sign;
pub(crate) mod ugc;
pub(crate) mod character;
pub(crate) mod quickmatch;
pub(crate) mod breakin;
pub(crate) mod player_equipments;

use std::{error::Error, io::{self, Read, Write}};

use fnrpc::{serialize, PayloadType, RequestParams, ResponseParams};
use crate::util::read_as_type;

use crate::client::ProtocolError;

pub type HandlerResult = Result<ResponseParams, Box<dyn Error>>;

pub fn get_payload_type(payload: &[u8]) -> Result<PayloadType, io::Error> {
    let mut reader = std::io::Cursor::new(payload);

    Ok(read_as_type::<u8>(&mut reader)?.into())
}

pub fn create_handling_context(
    payload: &[u8],
) -> Result<(ResponseContext, RequestParams), crate::client::ClientError> {
    let mut reader = io::Cursor::new(payload);

    let payload_type: PayloadType = read_as_type::<u8>(&mut reader)
        .map_err(crate::client::ClientError::Io)?.into();

    // TODO: newtype to remove runtime checking
    if payload_type != PayloadType::Request {
        return Err(crate::client::ClientError::Protocol(
            ProtocolError::WrongPayloadType,
        ));
    }

    let context = ResponseContext {
        sequence: read_as_type::<u32>(&mut reader)
            .map_err(crate::client::ClientError::Io)?,
    };

    let mut params_buffer = vec![];
    reader.read_to_end(&mut params_buffer)
        .map_err(crate::client::ClientError::Io)?;

    let params = fnrpc::deserialize::<RequestParams>(&params_buffer)
        .map_err(crate::client::ClientError::Wire)?.into();

    Ok((context, params))
}

#[derive(Debug)]
pub struct ResponseContext {
    pub sequence: u32,
}

impl ResponseContext {
    pub fn create_response_session(&self, params: ResponseParams) -> Result<Vec<u8>, io::Error> {
        let mut b = std::io::Cursor::new(vec![]);

        // Response payload type
        b.write_all(&u8::to_le_bytes(0x5))?;

        // Sequence number
        b.write_all(&u32::to_le_bytes(self.sequence))?;

        // Status
        b.write_all(&u8::to_le_bytes(0x1))?;

        let param_buffer = serialize(params)
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

        b.write_all(param_buffer.as_slice())?;

        Ok(b.into_inner())
    }


    /// Formats a response to be send to the client.
    pub fn create_response(&self, response: HandlerResult) -> Result<Vec<u8>, io::Error> {
        let mut b = std::io::Cursor::new(vec![]);

        // Response payload type
        b.write_all(&u8::to_le_bytes(0x5))?;
        // Sequence number
        b.write_all(&u32::to_le_bytes(self.sequence))?;

        match response {
            Err(e) => {
                log::error!("Could not handle request: {e:?}");

                // Status
                b.write_all(&u8::to_le_bytes(0x0))?;
                // Error code
                b.write_all(&u32::to_le_bytes(0))?;
            },
            Ok(params) => {
                // Status
                b.write_all(&u8::to_le_bytes(0x1))?;

                let param_buffer = serialize(params)
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

                b.write_all(param_buffer.as_slice())?;
            },
        };

        Ok(b.into_inner())
    }
}

fn encode_external_id(external_id: &str) -> String {
    format!(
        "{:x}",
        u64::from_str_radix(external_id, 10).unwrap()
    )
}
