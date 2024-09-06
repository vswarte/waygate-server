use byteorder::{ReadBytesExt, LE};
use waygate_message::armoredcore6::{ResponseParams, RequestParams};
use waygate_wire::deserialize;
use std::{env, error::Error, io::Read};

fn main() {
    let files: Vec<String> = env::args().skip(1).collect();

    files.iter()
        .for_each(|path| {
            println!("Parsing: {path}");

            let file = std::fs::File::open(path).unwrap();
            let message = parse_message(file).unwrap();
            match message {
                ParsedMessage::Request(_)
                | ParsedMessage::Response(_)
                | ParsedMessage::Push => println!("{:#?}", message),

                ParsedMessage::Heartbeat => {},
                ParsedMessage::Unknown => println!("Got unknown message type (might be KX)."),
            };
        });
}

fn parse_message<R: Read>(mut message: R) -> Result<ParsedMessage, Box<dyn Error>> {
    Ok(match message.read_u8().unwrap() {
        4 => ParsedMessage::Request(parse_request(message).unwrap()),
        5 => ParsedMessage::Response(parse_response(message).unwrap()),
        6 => unimplemented!(),
        7 => ParsedMessage::Heartbeat,
        _ => ParsedMessage::Unknown,
    })
}

#[derive(Debug)]
enum ParsedMessage {
    Request(ParsedRequest),
    Response(ParsedResponse),
    Push,
    Heartbeat,
    Unknown,
}

#[derive(Debug)]
struct ParsedRequest {
    sequence: u32,
    params: RequestParams,
}

fn parse_request<R: Read>(mut message: R) -> Result<ParsedRequest, Box<dyn Error>> {
    let sequence = message.read_u32::<LE>()?;

    let mut buf = vec![];
    message.read_to_end(&mut buf)?;

    Ok(ParsedRequest {
        sequence,
        params: deserialize(buf.as_slice())?,
    })
}

#[derive(Debug)]
struct ParsedResponse {
    sequence: u32,
    status: u8,
    body: ParsedResponseBody,
}

#[derive(Debug)]
enum ParsedResponseBody {
    Success(ResponseParams),
    Error(u32),
}

fn parse_response<R: Read>(mut message: R) -> Result<ParsedResponse, Box<dyn Error>> {
    let sequence = message.read_u32::<LE>()?;
    let status = message.read_u8()?;

    let body = if status != 0x1 {
        ParsedResponseBody::Error(message.read_u32::<LE>()?)
    } else {
        let mut buf = vec![];
        message.read_to_end(&mut buf)?;

        ParsedResponseBody::Success(deserialize(buf.as_slice())?)
    };

    Ok(ParsedResponse {
        sequence,
        status,
        body,
    })
}
