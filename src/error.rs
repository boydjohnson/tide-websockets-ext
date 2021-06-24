use crate::websocket_state::WebSocketHandle;

#[derive(Debug)]
pub enum WebSocketStateError {
    Timeout(WebSocketHandle),
    WebSocketError(tide_websockets::Error, WebSocketHandle),
    HttpError(http_types::Error, WebSocketHandle),
}

impl std::error::Error for WebSocketStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Timeout(_) => None,
            Self::WebSocketError(e, _) => Some(e),
            Self::HttpError(_, _) => None,
        }
    }
}

impl std::fmt::Display for WebSocketStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout(h) => write!(f, "Timeout {}", h),
            Self::WebSocketError(e, h) => write!(f, "{}: websocket client {}", e, h),
            Self::HttpError(e, h) => write!(f, "{}: websocket client {}", e, h),
        }
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
