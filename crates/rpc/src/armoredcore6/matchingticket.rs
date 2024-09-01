use waygate_message::armoredcore6::*;

use crate::HandlerResult;

pub async fn handle_create_matching_ticket() -> HandlerResult {
    Ok(ResponseParams::CreateMatchingTicket)
}

pub async fn handle_poll_matching_ticket() -> HandlerResult {
    Ok(ResponseParams::PollMatchingTicket(ResponsePollMatchingTicketParams {
        unk0: 0
    }))
}
