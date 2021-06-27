#![deny(missing_docs)]
//! A crate that makes it easier to work with `tide-websockets`.

pub mod error;
pub mod websocket_state;

pub use error::{Result, WebSocketStateError};
pub use websocket_state::{WebSocketHandle, WebSocketState};
