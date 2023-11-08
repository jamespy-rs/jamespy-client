pub mod app;
pub mod crossterm;
mod event_handlers;
pub mod ui;

use futures_util::StreamExt;
use serde_derive::{Deserialize, Serialize};
use std::{error::Error, sync::mpsc, time::Duration};
use tokio_tungstenite::{connect_async, WebSocketStream};
#[derive(Serialize, Deserialize, Debug)]
enum WebSocketEvent {
    NewMessage {
        message: serenity::model::channel::Message,
        guild_name: String,
        channel_name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080")
        .await
        .expect("Failed to connect to WebSocket server");

    let (sender, receiver) = mpsc::channel();
    let ws_sender = sender.clone();
    tokio::spawn(async move {
        handle_websocket_events(ws_stream, ws_sender).await;
    });

    let tick_rate = Duration::from_millis(25);
    crate::crossterm::run(tick_rate, receiver)?;
    Ok(())
}

async fn handle_websocket_events(
    mut ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ws_sender: mpsc::Sender<String>,
) {
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                let _ = ws_sender.send(msg.to_string());
            }
            Err(_err) => {
                // Handle error in the future dumbass.
            }
        }
    }
}
