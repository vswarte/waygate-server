use std::{collections::HashMap, fs::File, sync::mpsc::Sender};

use message::eldenring::{
    ObjectIdentifier, RequestGetAnnounceMessageListParams, RequestParams,
    ResponseGetAnnounceMessageListParams, ResponseGetAnnounceMessageListParamsEntry,
    ResponseParams, ResponsePollMatchingTicketParams,
};

mod announcement;
mod bloodmessage;
mod bloodstain;
mod breakin;
mod ghostdata;
mod match_density;
mod matchingticket;
mod player;
mod player_equipments;
mod quickmatch;
mod sign;
mod visit;

use crate::{
    handler::eldenring::announcement::AnnouncementConfig,
    handler::{HandleRequest, RequestHandler},
    logging::LogContext,
    notification::NotificationChannelPoolToken,
    protocol::ClientSession,
    services::eldenring::{
        breakin::BreakInPoolToken, quickmatch::QuickMatchPoolToken, sign::SignPoolToken,
        visit::VisitorPoolToken, GameServices,
    },
};

pub struct DefaultClientHandler<'a> {
    pub services: &'a GameServices,
    pub push_tx: Sender<Vec<u8>>,
    pub session: ClientSession,

    pub sign_tokens: HashMap<ObjectIdentifier, SignPoolToken<'a>>,
    pub breakin_token: Option<BreakInPoolToken<'a>>,
    pub quickmatch_token: Option<QuickMatchPoolToken<'a>>,
    pub visitor_token: Option<VisitorPoolToken<'a>>,

    _notification_token: NotificationChannelPoolToken<'a>,
}

impl<'a> DefaultClientHandler<'a> {
    pub fn new(
        services: &'a GameServices,
        push_tx: Sender<Vec<u8>>,
        session: ClientSession,
    ) -> Self {
        let _notification_token = services
            .notifications
            .insert(session.player_id, push_tx.clone());

        Self {
            services,
            push_tx,
            session,
            sign_tokens: Default::default(),
            breakin_token: Default::default(),
            quickmatch_token: Default::default(),
            visitor_token: Default::default(),
            _notification_token,
        }
    }
}

impl RequestHandler<RequestParams, ResponseParams> for DefaultClientHandler<'_> {
    async fn dispatch_request(
        &mut self,
        request: &RequestParams,
    ) -> Result<Option<ResponseParams>, Box<dyn std::error::Error>> {
        LogContext::insert("player_id", self.session.player_id.to_string());

        let result = match request {
            RequestParams::DeleteSession => ResponseParams::DeleteSession,

            RequestParams::GetAnnounceMessageList(request) => {
                ResponseParams::GetAnnounceMessageList(self.handle(request).await?)
            }

            RequestParams::CreateMatchingTicket(request) => {
                ResponseParams::CreateMatchingTicket(self.handle(request).await?)
            }

            RequestParams::PollMatchingTicket(request) => {
                ResponseParams::PollMatchingTicket(self.handle(request).await?)
            }

            RequestParams::DeleteMatchingTicket(request) => {
                ResponseParams::DeleteMatchingTicket(self.handle(request).await?)
            }

            RequestParams::UpdateLoginPlayerCharacter(request) => {
                ResponseParams::UpdateLoginPlayerCharacter(self.handle(request).await?)
            }

            RequestParams::UpdatePlayerStatus(request) => {
                ResponseParams::UpdatePlayerStatus(self.handle(request).await?)
            }

            RequestParams::CreateGhostData(request) => {
                ResponseParams::CreateGhostData(self.handle(request).await?)
            }

            RequestParams::GetGhostDataList(request) => {
                ResponseParams::GetGhostDataList(self.handle(request).await?)
            }

            RequestParams::CreateBloodstain(request) => {
                ResponseParams::CreateBloodstain(self.handle(request).await?)
            }

            RequestParams::GetBloodstainList(request) => {
                ResponseParams::GetBloodstainList(self.handle(request).await?)
            }

            RequestParams::GetDeadingGhost(request) => {
                ResponseParams::GetDeadingGhost(self.handle(request).await?)
            }

            RequestParams::CreateBloodMessage(request) => {
                ResponseParams::CreateBloodMessage(self.handle(request).await?)
            }

            RequestParams::GetBloodMessageList(request) => {
                ResponseParams::GetBloodMessageList(self.handle(request).await?)
            }

            RequestParams::EvaluateBloodMessage(request) => {
                ResponseParams::EvaluateBloodMessage(self.handle(request).await?)
            }

            RequestParams::RemoveBloodMessage(request) => {
                ResponseParams::RemoveBloodMessage(self.handle(request).await?)
            }

            RequestParams::ReentryBloodMessage(request) => {
                ResponseParams::ReentryBloodMessage(self.handle(request).await?)
            }

            RequestParams::GrUploadPlayerEquipments(request) => {
                ResponseParams::GrUploadPlayerEquipments(self.handle(request).await?)
            }

            RequestParams::GrGetPlayerEquipments(request) => {
                ResponseParams::GrGetPlayerEquipments(self.handle(request).await?)
            }

            RequestParams::CreateSign(request) => {
                ResponseParams::CreateSign(self.handle(request).await?)
            }

            RequestParams::CreateMatchAreaSign(request) => {
                ResponseParams::CreateMatchAreaSign(self.handle(request).await?)
            }

            RequestParams::UpdateSign(request) => {
                ResponseParams::UpdateSign(self.handle(request).await?)
            }

            RequestParams::GetSignList(request) => {
                ResponseParams::GetSignList(self.handle(request).await?)
            }

            RequestParams::GetMatchAreaSignList(request) => {
                ResponseParams::GetMatchAreaSignList(self.handle(request).await?)
            }

            RequestParams::RemoveSign(request) => {
                ResponseParams::RemoveSign(self.handle(request).await?)
            }

            RequestParams::SummonSign(request) => {
                ResponseParams::SummonSign(self.handle(request).await?)
            }

            RequestParams::RejectSign(request) => {
                ResponseParams::RejectSign(self.handle(request).await?)
            }

            RequestParams::GetBreakInTargetList(request) => {
                ResponseParams::GetBreakInTargetList(self.handle(request).await?)
            }

            RequestParams::BreakInTarget(request) => {
                ResponseParams::BreakInTarget(self.handle(request).await?)
            }

            RequestParams::AllowBreakInTarget(request) => {
                ResponseParams::AllowBreakInTarget(self.handle(request).await?)
            }

            RequestParams::RejectBreakInTarget(request) => {
                ResponseParams::RejectBreakInTarget(self.handle(request).await?)
            }

            RequestParams::SearchQuickMatch(request) => {
                ResponseParams::SearchQuickMatch(self.handle(request).await?)
            }

            RequestParams::RegisterQuickMatch(request) => {
                ResponseParams::RegisterQuickMatch(self.handle(request).await?)
            }

            RequestParams::UnregisterQuickMatch(request) => {
                ResponseParams::UnregisterQuickMatch(self.handle(request).await?)
            }

            RequestParams::UpdateQuickMatch(request) => {
                ResponseParams::UpdateQuickMatch(self.handle(request).await?)
            }

            RequestParams::JoinQuickMatch(request) => {
                ResponseParams::JoinQuickMatch(self.handle(request).await?)
            }

            RequestParams::AcceptQuickMatch(request) => {
                ResponseParams::AcceptQuickMatch(self.handle(request).await?)
            }

            RequestParams::QuickMatchResultLog => ResponseParams::QuickMatchResultLog,

            RequestParams::QuickMatchEndLog => ResponseParams::QuickMatchEndLog,

            RequestParams::CreateBattleSession(request) => {
                ResponseParams::CreateBattleSession(self.handle(request).await?)
            }

            RequestParams::GetMatchDensity(request) => {
                ResponseParams::GetMatchDensity(self.handle(request).await?)
            }

            RequestParams::GetVisitorList(request) => {
                ResponseParams::GetVisitorList(self.handle(request).await?)
            }

            RequestParams::Visit(request) => ResponseParams::Visit(self.handle(request).await?),

            RequestParams::RejectVisit(request) => {
                ResponseParams::RejectVisit(self.handle(request).await?)
            }

            _ => {
                log::warn!(
                    context:serde = LogContext::current(),
                    request_type = request.name(),
                    request:serde = request;
                    "Request without handler fn. request = {}", request.name()
                );

                return Ok(None);
            }
        };
        log::info!(
            context:serde = LogContext::current(),
            request_type = request.name(),
            request:serde = request,
            response:serde = result;
            "Request processed successfully.",
        );
        Ok(Some(result))
    }
}

#[derive(Default)]
pub struct BannedClientHandler {}

impl RequestHandler<RequestParams, ResponseParams> for BannedClientHandler {
    async fn dispatch_request(
        &mut self,
        request: &RequestParams,
    ) -> Result<Option<ResponseParams>, Box<dyn std::error::Error>> {
        let response = match request {
            RequestParams::GetAnnounceMessageList(request) => {
                ResponseParams::GetAnnounceMessageList(self.handle(request).await?)
            }
            RequestParams::PollMatchingTicket(_) => {
                ResponseParams::PollMatchingTicket(ResponsePollMatchingTicketParams { unk0: 0 })
            }
            RequestParams::DeleteSession => ResponseParams::DeleteSession,
            _ => {
                log::warn!(
                    context:serde = LogContext::current(),
                    request_type = request.name(),
                    request:serde = request;
                    "Banned client attempted to make an unsupported request."
                );
                return Ok(None);
            }
        };
        log::info!(
            context:serde = LogContext::current(),
            request_type = request.name(),
            request:serde = request,
            response:serde = &response;
            "Banned client request processed successfully.",
        );
        Ok(Some(response))
    }
}

impl HandleRequest<Box<RequestGetAnnounceMessageListParams>, ResponseGetAnnounceMessageListParams>
    for BannedClientHandler
{
    async fn handle(
        &mut self,
        _request: &Box<RequestGetAnnounceMessageListParams>,
    ) -> Result<ResponseGetAnnounceMessageListParams, Box<dyn std::error::Error>> {
        let announcements = serde_yaml::from_reader::<_, AnnouncementConfig>(File::open(
            "config/ban_announcement.yml",
        )?)?;

        Ok(ResponseGetAnnounceMessageListParams {
            changes: announcements
                .changes
                .iter()
                .map(ResponseGetAnnounceMessageListParamsEntry::from)
                .collect(),
            notices: announcements
                .notices
                .iter()
                .map(ResponseGetAnnounceMessageListParamsEntry::from)
                .collect(),
        })
    }
}

pub enum ActiveHandler<'a> {
    Default(DefaultClientHandler<'a>),
    Banned(BannedClientHandler),
}
