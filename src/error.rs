//! WebSocketState error type

use crate::websocket_state::WebSocketHandle;

/// WebSocketState error type
#[derive(Debug)]
pub enum WebSocketStateError {
    /// Timeout
    Timeout(WebSocketHandle),
    /// WebSocketError
    WebSocketError(tide_websockets::Error, WebSocketHandle),
    /// HttpError
    HttpError(http_types::Error, WebSocketHandle),
    /// State does not contain that `WebSocketConnection`
    NoSuchWebSocketClient(WebSocketHandle),
}

impl std::error::Error for WebSocketStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Timeout(_) => None,
            Self::WebSocketError(e, _) => Some(e),
            Self::HttpError(_, _) => None,
            Self::NoSuchWebSocketClient(_) => None,
        }
    }
}

impl std::fmt::Display for WebSocketStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout(h) => write!(f, "Timeout {}", h),
            Self::WebSocketError(e, h) => write!(f, "{}: websocket client {}", e, h),
            Self::HttpError(e, h) => write!(f, "{}: websocket client {}", e, h),
            Self::NoSuchWebSocketClient(h) => write!(f, "No such websocket handle: {}", h),
        }
    }
}

/// WebSocketState Result type
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
