#![allow(unused, dead_code)]

pub mod reader;
pub mod builder;

pub mod session;
pub mod eldenring;

#[repr(u8)]
#[derive(Debug)]
pub enum MessageType {
    Request = 0x4,
    Response = 0x5,
    Push = 0x6,
    Heartbeat = 0x7,
}
