use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::stream::StreamExt;

use crate::json::{CCPacket, Position};

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket))
}

pub async fn websocket(stream: WebSocket) {
    // By splitting, we can send and receive at the same time.
    let (mut _sender, mut receiver) = stream.split();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(message) = message {
            if let Ok(packet) = serde_json::from_str::<CCPacket>(&message) {
                match packet {
                    CCPacket::ClientConnect { name, id } => {
                        println!("Client connected! {name} {id}");
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
                }
            }
        }
    }
}
