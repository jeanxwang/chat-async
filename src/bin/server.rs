use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Send a welcome message to the client
    ws_stream
        .send(Message::text("Welcome to chat! Type a message".to_string()))
        .await?;

    // Subscribe to the broadcast channel to receive messages from other clients
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive messages from the client
            Some(Ok(msg)) = ws_stream.next() => {
                if let Some(text) = msg.as_text() {
                    println!("From client {addr:?}: {text}", addr = addr, text = text);
                    // Broadcast the received message to other clients
                    bcast_tx.send(format!("{addr} : {text}", addr = addr, text = text))
                        .map_err(|e| -> tokio::sync::broadcast::error::SendError<String> { e.into() })?;
                }
            }
            // Receive messages from other clients via broadcast channel
            result = bcast_rx.recv() => {
                if let Ok(msg) = result {
                    // Send the received message to the client
                    ws_stream.send(Message::text(msg)).await?;
                } else {
                    return Err("Error receiving message from broadcast channel".into());
                }
            }
            // If the client stream is closed, return Ok to close the connection
            else => return Ok(()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}