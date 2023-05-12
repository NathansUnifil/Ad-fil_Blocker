use hyper::{service::service_fn, Body, Client, Request, Response, Server};
use std::net::SocketAddr;
use tower::make::Shared;
use tokio::select;
use tokio_util::sync::CancellationToken;

#[tokio::main]

async fn main() {

	let make_service = Shared::new(service_fn(log));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = Server::bind(&addr).serve(make_service);

	if let Err(e) = server.await {
		println!("error: {}", e);
	}
}

async fn handle(req: Request<Body>) -> hyper::Result<Response<Body>> {

	let token = CancellationToken::new();
	let cloned_token = token.clone();

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

	let join_handle = tokio::spawn(async move {
		if path.chars().any(filtros) {
			Response::new(Body::from("Você foi cancelado!"));
			select! {
			_ = cloned_token.cancelled() => {
				5
				}
			//_ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                //99
			//}
		}
		}
	} 5 );

	tokio::spawn(async move {
		tokio::time::sleep(std::time::Duration::from_millis(10)).await;
		token.cancel();
	});


	assert_eq!(5, join_handle.await.unwrap());

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