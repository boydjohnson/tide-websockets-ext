use crate::websocket_state::WebSocketHandle;

#[derive(Debug)]
pub enum WebSocketStateError {
    Timeout(WebSocketHandle),
    WebSocketError(tide_websockets::Error, WebSocketHandle),
    HttpError(http_types::Error, WebSocketHandle),
}

impl std::error::Error for WebSocketStateError {
    fn description(&self) -> &str {
        todo!();
    }
}

impl std::fmt::Display for WebSocketStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, WebSocketStateError>;

impl From<(tide_websockets::Error, WebSocketHandle)> for WebSocketStateError {
    fn from(other: (tide_websockets::Error, WebSocketHandle)) -> Self {
        Self::WebSocketError(other.0, other.1)
    }
}

impl From<(http_types::Error, WebSocketHandle)> for WebSocketStateError {
    fn from(other: (http_types::Error, WebSocketHandle)) -> Self {
        Self::HttpError(other.0, other.1)
    }
}

impl From<(async_std::future::TimeoutError, WebSocketHandle)> for WebSocketStateError {
    fn from(other: (async_std::future::TimeoutError, WebSocketHandle)) -> Self {
        Self::Timeout(other.1)
    }
}
