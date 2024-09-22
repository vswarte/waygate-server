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
pub(crate) mod room;

pub use handler::handle_request;

use std::error::Error;

use waygate_connection::ClientError;
use waygate_message::ResponseParams;

type HandlerResult = Result<ResponseParams, Box<dyn Error>>;
