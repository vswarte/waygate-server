use thiserror::Error;
use tungstenite::Message;
use tokio::net::TcpStream;
use futures_util::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::WebSocketStream;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Could not accept connection")]
    AcceptFailed, 

    #[error("Could not read from connection: {0}")]
    ReadFailed(tungstenite::Error),

    #[error("Could not write to connection: {0}")]
    WriteFailed(tungstenite::Error),

    #[error("Transport was closed")]
    TransportClosed,
}

type ClientTransportDriver = WebSocketStream<TcpStream>;
pub type ClientTransportSink = SplitSink<ClientTransportDriver, Message>;
pub type ClientTransportStream = SplitStream<ClientTransportDriver>;

#[derive(Debug)]
pub struct ClientTransport {
    pub sink: ClientTransportSink, 
    pub stream: ClientTransportStream, 
}

impl ClientTransport {
    pub async fn receive_next(&mut self) -> Result<Message, TransportError> {
        self.stream.next()
            .await.ok_or(TransportError::TransportClosed)?
            .map_err(TransportError::ReadFailed)
    }
}

impl From<WebSocketStream<TcpStream>> for ClientTransport {
    fn from(value: WebSocketStream<TcpStream>) -> Self {
        let (sink, stream) = value.split();
        Self { sink, stream }
    }
}
