use std::{io::Write, marker::PhantomData};

use byteorder::{WriteBytesExt, LE};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing data for field. {0}")]
    MissingField(&'static str),
    #[error("Wire. {0}")]
    Wire(#[from] wire::FNWireError),
    #[error("Io. {0}")]
    Io(#[from] std::io::Error),
}

pub struct MessageBuilder {}

impl MessageBuilder
{
    /// Build a new response message.
    pub fn response<R>() -> ResponseMessageBuilder<R> where R: serde::Serialize {
        ResponseMessageBuilder { sequence: None, error: None, body: None }
    }

    /// Build a new push message.
    pub fn push<R>() -> PushMessageBuilder<R> where R: serde::Serialize {
        PushMessageBuilder { body: None }
    }

    /// Build a heartbeat message
    pub fn heartbeat() -> &'static [u8] {
        &[0x07, 0x64]
    }
}

pub struct ResponseMessageBuilder<R>
where
    R: serde::Serialize,
{
    sequence: Option<u32>,
    error: Option<u32>,
    body: Option<R>,
}

impl<R> ResponseMessageBuilder<R>
where
    R: serde::Serialize,
{
    pub fn sequence(self, sequence: u32) -> Self {
        Self {
            sequence: Some(sequence),
            error: self.error,
            body: self.body,
        }
    }

    pub fn body(self, body: R) -> Self {
        Self {
            sequence: self.sequence,
            error: self.error,
            body: Some(body),
        }
    }

    pub fn error(self, error: u32) -> Self {
        Self {
            sequence: self.sequence,
            error: Some(error),
            body: self.body,
        }
    }

    pub fn build(self) -> Result<Vec<u8>, Error> {
        let sequence = self.sequence.ok_or(Error::MissingField("sequence"))?;

        let mut result = vec![];
        // Response message type
        result.write_u8(5)?;
        // Write sequence so client knows what message this is a response for.
        result.write_u32::<LE>(sequence)?;

        if let Some(error) = self.error {
            // Write error status
            result.write_u8(0)?;
            // Write error code
            result.write_u32::<LE>(0)?;
        } else {
            // Write success status
            result.write_u8(1)?;

            // Write serialized body
            let body = self.body.ok_or(Error::MissingField("body"))?;
            let body = wire::serialize(body)?;
            result.write_all(&body)?;
        }

        Ok(result)
    }
}

pub struct PushMessageBuilder<R>
where
    R: serde::Serialize,
{
    body: Option<R>,
}

impl<R> PushMessageBuilder<R>
where
    R: serde::Serialize,
{
    pub fn body(self, body: R) -> Self {
        Self { body: Some(body) }
    }

    pub fn build(self) -> Result<Vec<u8>, Error> {
        let mut result = vec![];

        // Response message type
        result.write_u8(6)?;
        // ???
        result.write_u8(0)?;

        let body = self.body.ok_or(Error::MissingField("body"))?;
        let body = wire::serialize(body)?;
        result.write_all(&body)?;

        Ok(result)
    }
}
