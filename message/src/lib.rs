#![allow(unused, dead_code)]

pub mod builder;
pub mod reader;

pub mod eldenring;
pub mod session;

#[repr(u8)]
#[derive(Debug)]
pub enum MessageType {
    Request = 0x4,
    Response = 0x5,
    Push = 0x6,
    Heartbeat = 0x7,
}
