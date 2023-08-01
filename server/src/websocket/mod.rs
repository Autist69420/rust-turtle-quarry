use std::sync::{Arc, Mutex};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse, Extension,
};
use futures::{stream::StreamExt, SinkExt};
use tracing::info;

use crate::{json::{CCPacket, Position, ServerPacket}, AppState};

pub async fn websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

pub async fn websocket(stream: WebSocket, _state: Arc<Mutex<AppState>>) {
    // By splitting, we can send and receive at the same time.
    let (mut _sender, mut receiver) = stream.split();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(message) = message {
            if let Ok(packet) = serde_json::from_str::<CCPacket>(&message) {
                match packet {
                    CCPacket::ClientConnect { name, id, gps } => {
                        info!("Client connected: name={name}; id={id}; gps={gps}");
                    }
                    CCPacket::LevelDecrease {
                        old_level,
                        new_level,
                        position,
                    } => {}
                    CCPacket::LevelIncrease {
                        old_level,
                        new_level,
                        position,
                    } => {}
                    CCPacket::UpdatePosition { new_position, old_position } => todo!(),
                }
            }
        }
    }
}
