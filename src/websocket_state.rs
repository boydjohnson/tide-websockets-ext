#[cfg(feature = "async-std")]
use async_lock::RwLock;
use std::{sync::Arc, time::Duration, collections::BTreeMap};
use tide_websockets::WebSocketConnection;
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct WebSocketHandle(String);

impl WebSocketHandle {
    fn random() -> Self {
        WebSocketHandle(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone)]
pub struct WebSocketState(Arc<RwLock<BTreeMap<WebSocketHandle, WebSocketConnection>>>);

impl Default for WebSocketState {
    fn default() -> Self {
        WebSocketState(Arc::new(RwLock::new(BTreeMap::default())))
    }
}

impl WebSocketState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, conn: &WebSocketConnection) -> WebSocketHandle {
        let handle = WebSocketHandle::random();
        self.0.write().await.insert(handle.clone(), conn.clone());
        handle
    }

    pub async fn send_all_string(
        &self,
        handle: &WebSocketHandle,
        msg: String,
        timeout: Duration,
    ) -> crate::error::Result<()> {

        Ok(())
    }
}
