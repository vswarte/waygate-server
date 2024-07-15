#[derive(Debug, PartialEq)]
pub enum PayloadType {
    Request,
    Response,
    Push,
    Heartbeat,
    Unknown(u8)
}

impl From<u8> for PayloadType {
    fn from(value: u8) -> Self {
        match value {
            0x4 => Self::Request,
            0x5 => Self::Response,
            0x6 => Self::Push,
            0x7 => Self::Heartbeat,
            _   => Self::Unknown(value),
        }
    }
}
