use async_lock::RwLock;
use async_std::prelude::FutureExt;
use futures::{stream::FuturesUnordered, StreamExt};
use serde::Serialize;
use std::{collections::BTreeMap, future::Future, sync::Arc, time::Duration};
use tide_websockets::{Message, WebSocketConnection};
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
        msg: String,
        timeout: Duration,
    ) -> Vec<crate::error::Result<()>> {
        let func =
            |conn: WebSocketConnection, msg: String| async move { conn.send_string(msg).await };

        self.send_all(func, msg, timeout).await
    }

    pub async fn send_all_json<T: Serialize + Clone>(
        &self,
        msg: T,
        timeout: Duration,
    ) -> Vec<crate::error::Result<()>> {
        let func = |conn: WebSocketConnection, msg: T| async move { conn.send_json(&msg).await };

        self.send_all(func, msg, timeout).await
    }

    pub async fn send_all_msg(
        &self,
        msg: Message,
        timeout: Duration,
    ) -> Vec<crate::error::Result<()>> {
        let func = |conn: WebSocketConnection, msg: Message| async move { conn.send(msg).await };

        self.send_all(func, msg, timeout).await
    }

    pub async fn send_all_bytes(
        &self,
        bytes: Vec<u8>,
        timeout: Duration,
    ) -> Vec<crate::error::Result<()>> {
        let func =
            |conn: WebSocketConnection, item: Vec<u8>| async move { conn.send_bytes(item).await };

        self.send_all(func, bytes, timeout).await
    }

    async fn send_all<
        E,
        K: Clone,
        T: Future<Output = Result<(), E>>,
        F: Fn(WebSocketConnection, K) -> T,
    >(
        &self,
        func: F,
        item: K,
        timeout: Duration,
    ) -> Vec<crate::error::Result<()>>
    where
        crate::error::WebSocketStateError: From<(E, WebSocketHandle)>,
    {
        self.0
            .read()
            .await
            .iter()
            .map(move |(handle, conn)| {
                Self::send_with_timeout(func(conn.clone(), item.clone()), handle, timeout)
            })
            .collect::<FuturesUnordered<_>>()
            .collect()
            .await
    }

    async fn send_with_timeout<E, F: std::future::Future<Output = Result<(), E>>>(
        fut: F,
        handle: &WebSocketHandle,
        timeout: Duration,
    ) -> crate::error::Result<()>
    where
        crate::error::WebSocketStateError: From<(E, WebSocketHandle)>,
    {
        Ok(fut
            .timeout(timeout)
            .await
            .map_err(|e| (e, handle.clone()))?
            .map_err(|e| (e, handle.clone()))?)
    }
}
