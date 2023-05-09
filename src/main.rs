use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread::Builder;
#[tokio::main]

use hyper::{service::{make_service_fn, service_fn}, Request, Body, Responce};
use hyper::header::SERVER;
use hyper::Response;
use hyper::server::conn::AddrIncoming;
use tokio::net::windows::named_pipe::PipeEnd::Client;
use tower::MakeService;
use tower::make::Shared;

async fn main() {
    let make_service = Shared::new(service_fn(handle()));
    let addr = SocketAddr::from([127, 0, 0, 1], 3000);
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e)
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    //Ok(Response::new(Body::from("Hello from HTTP proxy")))
    let client = Client::new();
    client.request(req).await

}

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API Path = {}", path);
    } else {
        println!("Generic path = {}", path)
    }

    handler(req).await
}