use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{stream::StreamExt};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket))
}

pub async fn websocket(stream: WebSocket) {
    // By splitting, we can send and receive at the same time.
    let (mut _sender, mut receiver) = stream.split();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(_message) = message {

        }
    }
}
