use waygate_message::*;

use crate::rpc;

// The numbers... They mean something.
pub async fn handle_create_matching_ticket() -> rpc::HandlerResult {
    Ok(ResponseParams::CreateMatchingTicket)
}

pub async fn handle_poll_matching_ticket() -> rpc::HandlerResult {
    Ok(ResponseParams::PollMatchingTicket(ResponsePollMatchingTicketParams {
        unk0: 0
    }))
}
