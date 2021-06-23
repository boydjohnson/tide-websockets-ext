#[derive(Debug)]
pub struct WebSocketStateError;

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
