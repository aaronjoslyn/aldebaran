mod build;
mod http;
mod watch;

#[tokio::main]
async fn main() {
    tokio::spawn(build::watch_wasm());
    http::listen_http().await;
}
