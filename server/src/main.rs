mod build;
mod http;

#[tokio::main]
async fn main() {
    tokio::spawn(build::watch_wasm());
    http::listen_http().await;
}
