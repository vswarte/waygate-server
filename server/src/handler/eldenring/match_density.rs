use message::eldenring::{RequestGetMatchDensityParams, ResponseGetMatchDensityParams};

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestGetMatchDensityParams>, ResponseGetMatchDensityParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestGetMatchDensityParams>,
    ) -> Result<ResponseGetMatchDensityParams, Box<dyn std::error::Error>> {
        let areas = vec![
            1100000, 1100001, 1100010, 1100011, 1100012, 1100013, 1100014, 1100015, 1100016,
            1100017, 1100018, 1100019, 1100021, 1100091,
        ];

        let blue_activity = areas
            .iter()
            .enumerate()
            .map(|(i, _)| (i % 9) as u8)
            .collect();

        let red_activity = areas
            .iter()
            .map(|_| 3)
            .collect();

        Ok(ResponseGetMatchDensityParams {
            areas,
            blue_activity,
            red_activity,
            unk1: 0,
        })
    }
}
