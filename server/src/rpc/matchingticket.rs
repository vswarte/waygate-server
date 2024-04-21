use fnrpc::ResponseParams;
use fnrpc::matchingticket::*;

use crate::rpc;
use crate::session::ClientSession;

pub async fn handle_create_matching_ticket(
    session: ClientSession,
    request: RequestCreateMatchingTicketParams,
) -> rpc::HandlerResult {
    log::info!("Player sent CreateMatchingTicket. player = {}", session.player_id);
    log::info!("Last 10 numbers: {:?}", &request.unk1[0..10]);

    Ok(ResponseParams::CreateMatchingTicket)
}
