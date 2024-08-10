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
pub(crate) mod matchingticket;

use std::{error::Error, io::{self, Read, Write}};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use waygate_message::{PayloadType, RequestParams, ResponseParams};

use crate::client::ProtocolError;

pub type HandlerResult = Result<ResponseParams, Box<dyn Error>>;

pub fn create_handling_context<R: Read>(mut r: R) -> Result<(ResponseContext, RequestParams), crate::client::ClientError> {
    let payload_type: PayloadType = r.read_u8()?.into();

    // TODO: newtype to remove runtime checking
    if payload_type != PayloadType::Request {
        return Err(crate::client::ClientError::Protocol(
            ProtocolError::WrongPayloadType,
        ));
    }

    let context = ResponseContext { sequence: r.read_u32::<LE>()? };

    let mut params_buffer = vec![];
    r.read_to_end(&mut params_buffer)?;

    let params = waygate_fnrpc::deserialize::<RequestParams>(&params_buffer)
        .map_err(crate::client::ClientError::Wire)?;

    Ok((context, params))
}

#[derive(Debug)]
pub struct ResponseContext {
    pub sequence: u32,
}

impl ResponseContext {
    pub fn create_response_session(&self, params: ResponseParams) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_u8(5)?; // Response payload type
        buffer.write_u32::<LE>(self.sequence)?;
        buffer.write_u8(1)?; // Status code

        let param_buffer = waygate_fnrpc::serialize(params)
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        buffer.write_all(param_buffer.as_slice())?;

        Ok(buffer)
    }

    /// Formats a response to be send to the client.
    pub fn create_response(&self, response: HandlerResult) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_u8(5)?; // Response payload type
        buffer.write_u32::<LE>(self.sequence)?;

        match response {
            Err(e) => {
                log::error!("Could not handle request: {e:?}");

                buffer.write_u8(0)?; // Status code
                buffer.write_u32::<LE>(0)?; // Error code
            },
            Ok(params) => {
                buffer.write_u8(1)?; // Status code
                let param_buffer = waygate_fnrpc::serialize(params)
                    .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
                buffer.write_all(param_buffer.as_slice())?;
            },
        };

        Ok(buffer)
    }
}
