use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = sanitise_path(req.uri().path());
    let expanded = expand_path(&path);
    let content_type = find_content_type(&expanded);
    match read_file(&expanded).await {
        Ok(file) => Ok(serve_file(file, content_type)),
        Err(_) => Ok(not_found()),
    }
}

fn serve_file(file: Vec<u8>, content_type: &str) -> Response<Body> {
    Response::builder()
        .header("Content-Type", content_type)
        .status(200)
        .body(file.into())
        .unwrap_or_else(|_| server_error())
}

fn server_error() -> Response<Body> {
    Response::builder().status(500).body(Body::empty()).unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder().status(404).body(Body::empty()).unwrap()
}

fn expand_path(path: &str) -> &str {
    match path {
        "" => "index.html",
        _ => path,
    }
}

async fn read_file(path: &str) -> Result<Vec<u8>> {
    let path = format!("./public/{}", &path);
    let res = tokio::fs::read(path).await?;
    Ok(res)
}

fn find_content_type(path: &str) -> &str {
    match std::path::Path::new(path).extension() {
        Some(path) => match path.to_str() {
            Some("css") => "text/css",
            Some("html") => "text/html",
            Some("ico") => "image/vnd.microsoft.icon",
            Some("js") => "text/javascript",
            Some("wasm") => "application/wasm",
            _ => "application/octet-stream",
        },
        _ => "application/octet-stream",
    }
}

fn sanitise_path(path: &str) -> String {
    path.replace("/", "").replace("..", "").replace("\\", "")
}

pub async fn listen() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service_fn = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req)))
    });
    let server = Server::bind(&addr).serve(service_fn);
    println!("Listening for HTTP on {}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
