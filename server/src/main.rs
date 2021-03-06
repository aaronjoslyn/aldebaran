mod build;
mod http;
mod watch;
mod websocket;

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(build::watch_wasm(tx));
    tokio::spawn(websocket::listen_websocket(rx));
    http::listen_http().await;
}
