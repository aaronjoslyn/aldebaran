mod http;

#[tokio::main]
async fn main() {
    http::listen().await;
}
