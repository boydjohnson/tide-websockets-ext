use async_std::prelude::*;
use std::time::Duration;
use tide::Request;
use tide_websockets::{Message, WebSocket};
use tide_websockets_ext::websocket_state::WebSocketState;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::with_level(tide::log::LevelFilter::Debug);

    let state = WebSocketState::default();

    let mut app = tide::with_state(state);

    app.at("/ws")
        .with(WebSocket::new(
            |request: Request<WebSocketState>, mut stream| async move {
                let handle = request.state().insert(&stream).await;

                while let Some(Ok(Message::Text(input))) = stream.next().await {
                    let output: String = input.chars().rev().collect();

                    request
                        .state()
                        .send_all_string(
                            format!("{} | {}", &input, &output),
                            Duration::from_millis(100),
                        )
                        .await;
                }

                request.state().delete(&handle).await;

                Ok(())
            },
        ))
        .get(|_| async move { Ok("this was not a websocket request") });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
