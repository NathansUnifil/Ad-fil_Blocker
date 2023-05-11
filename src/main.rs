use hyper::{service::service_fn, Body, Client, Request, Response, Server};
use std::net::SocketAddr;
use tower::make::Shared;
use tokio::select;

#[tokio::main]

async fn main() {

	/*

	let mut response = Response::new(Body::empty());

	match (req.method(), req.uri().path()) {
		(&Method::GET, "/") => {
			*response.body_mut() = Body::from("Try POSTing data to /echo");
		},
		(&Method::POST, "/echo") => {
			*response.body_mut() = req.into_body();
		},
		_ => {
			*response.status_mut() = StatusCode::NOT_FOUND;
		},

		(&Method::POST, "/echo/reverse") => {
			// Protect our server from massive bodies.
			let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
			if upper > 1024 * 64 {
				let mut resp = Response::new(Body::from("Body too big"));
				*resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
				return Ok();
			}

			// Await the full body to be concatenated into a single `Bytes`...
			let full_body = hyper::body::to_bytes(req.into_body()).await?;

			// Iterate the full body in reverse order and collect into a new Vec.
			let reversed = full_body.iter()
				.rev()
				.cloned()
				.collect::<Vec<u8>>();

			*response.body_mut() = reversed.into();
		},

	};

	Ok(response);

	 */


	let make_service = Shared::new(service_fn(log));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = Server::bind(&addr).serve(make_service);

	if let Err(e) = server.await {
		println!("error: {}", e);
	}
}

async fn handle(req: Request<Body>) -> hyper::Result<Response<Body>> {

	//Ok(Response::new(Body::from("Hello from HTTP proxy")))

	let filtros = ["*://*.doubleclick.net/*",
		"*://partner.googleadservices.com/*",
		"*://*.googlesyndication.com/*",
		"*://*.google-analytics.com/*",
		"*://creative.ak.fbcdn.net/*",
		"*://*.adbrite.com/*",
		"*://*.exponential.com/*",
		"*://*.quantserve.com/*",
		"*://*.scorecardresearch.com/*",
		"*://*.zedo.com/*",];

	let path = req.uri().path();

	if path.chars().any(filtros) {
		Response::new(Body::from("Você foi cancelado!"));
		select! {
			_ = filtros.cancelled() => {
				()
			}

		}
	}
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

	handle(req).await
}