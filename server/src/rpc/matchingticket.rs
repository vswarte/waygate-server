use fnrpc::ResponseParams;
use fnrpc::matchingticket::*;

use crate::rpc;
use crate::session::ClientSession;
use crate::session::ClientSessionContainer;

// The numbers... They mean something.
pub async fn handle_create_matching_ticket(
    session: ClientSession,
    request: RequestCreateMatchingTicketParams,
) -> rpc::HandlerResult {
    Ok(ResponseParams::CreateMatchingTicket)
}

pub async fn handle_poll_matching_ticket() -> rpc::HandlerResult {
    Ok(ResponseParams::PollMatchingTicket(ResponsePollMatchingTicketParams {
        unk0: 0
    }))
}
