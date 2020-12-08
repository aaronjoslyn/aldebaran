use anyhow::Result;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;

pub type WebSocketSink = SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>;
pub type WebSocketStream = SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>;
pub type WebSocketMessage = tokio_tungstenite::tungstenite::Message;

pub async fn listen_websocket(reload_rx: mpsc::Receiver<String>) -> Result<()> {
    let addr = "127.0.0.1:4000";
    let try_socket = TcpListener::bind(addr).await;
    let mut listener = try_socket.expect("failed to bind websocket server");
    println!("Listening for websockets on {}", addr);
    let listeners: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(vec![]));
    tokio::spawn(forward_reload(reload_rx, listeners.clone()));
    while let Ok((stream, _)) = listener.accept().await {
        let shared_listeners = listeners.clone();
        let (tx, rx) = mpsc::channel(100);
        let mut listeners = shared_listeners.lock().await;
        listeners.push(tx);
        tokio::spawn(accept_stream(stream, rx));
    }
    Ok(())
}

async fn forward_reload(
    mut reload_rx: mpsc::Receiver<String>,
    shared_listeners: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
) {
    while let Some(msg) = reload_rx.recv().await {
        let mut listeners = shared_listeners.lock().await;
        let mut failed = vec![];
        for (i, l) in listeners.iter().enumerate() {
            let mut listener = l.clone();
            match listener.send(msg.clone()).await {
                Ok(_) => (),
                Err(_) => failed.push(i),
            }
        }
        for i in failed {
            listeners.remove(i);
        }
    }
}

async fn accept_stream(stream: TcpStream, mut rx: mpsc::Receiver<String>) -> Result<()> {
    let (mut tx, _) = accept_websocket(stream).await?;
    println!("Websocket connected.");
    while let Some(msg) = rx.recv().await {
        tx.send(WebSocketMessage::text(msg)).await?;
    }
    Ok(())
}

async fn accept_websocket(stream: TcpStream) -> Result<(WebSocketSink, WebSocketStream)> {
    let local_client_stream = tokio_tungstenite::accept_async(stream).await?;
    Ok(local_client_stream.split())
}
