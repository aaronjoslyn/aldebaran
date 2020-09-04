use anyhow::Result;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

pub type WebSocketSink = SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>;
pub type WebSocketStream = SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>;
pub type WebSocketMessage = tokio_tungstenite::tungstenite::Message;

pub async fn listen_websocket(rx: watch::Receiver<String>) {
    let addr = "127.0.0.1:4000";
    let try_socket = TcpListener::bind(addr).await;
    let mut listener = try_socket.expect("failed to bind websocket server");
    println!("Listening for websockets on {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_stream(stream, rx.clone()));
    }
}

async fn accept_stream(stream: TcpStream, rx: watch::Receiver<String>) -> Result<()> {
    let (mut local_tx, _) = accept_websocket(stream).await?;
    let (build_tx, mut build_rx) = mpsc::channel(100);
    println!("Client connected.");
    tokio::spawn(proxy_build(rx, build_tx.clone()));
    while let Some(msg) = build_rx.next().await {
        local_tx.send(msg).await?;
    }
    Ok(())
}

async fn accept_websocket(stream: TcpStream) -> Result<(WebSocketSink, WebSocketStream)> {
    let local_client_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(local_client_stream.split())
}

async fn proxy_build(
    mut rx: watch::Receiver<String>,
    mut tx: mpsc::Sender<WebSocketMessage>,
) -> Result<()> {
    while let Some(msg) = rx.next().await {
        tx.send(WebSocketMessage::text(msg)).await?;
    }
    Ok(())
}
