use hyper::{Body, Client, Request, Response};
use reqwest;

#[tokio::main]

async fn main() {

	let filtros=["*://*.doubleclick.net/*",
	"*://partner.googleadservices.com/*",
	"*://*.googlesyndication.com/*",
	"*://*.google-analytics.com/*",
	"*://creative.ak.fbcdn.net/*",
	"*://*.adbrite.com/*",
	"*://*.exponential.com/*",
	"*://*.quantserve.com/*",
	"*://*.scorecardresearch.com/*",
	"*://*.zedo.com/*",];

	let mut client = hyper::Client::new();

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	// 2

	// Pass our request builder object to our client.
	let hyperresp1 = client.request(req1).await?;

	let req1 = hyper::Request::builder()
		.method(hyper::Method::GET)
		.uri("*://*.doubleclick.net/*")
		.header("user-agent", "the-awesome-agent/007")
		.body(hyper::Body::from(""))?;

	// Get the response body bytes.
	let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

	// Convert the body bytes to utf-8
	let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

	let resp1 = match reqwest::blocking::get(body1) {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp1);
	let resp2 = match reqwest::blocking::get("*://partner.googleadservices.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp2);
	let resp3 = match reqwest::blocking::get("*://*.googlesyndication.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp3);
	let resp4 = match reqwest::blocking::get("*://*.google-analytics.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp4);
	let resp5 = match reqwest::blocking::get("*://creative.ak.fbcdn.net/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp5);
	let resp6 = match reqwest::blocking::get("*://*.adbrite.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp6);
	let resp7 = match reqwest::blocking::get("*://*.exponential.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp7);
	let resp8 = match reqwest::blocking::get("*://*.quantserve.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp8);
	let resp9 = match reqwest::blocking::get("*://*.scorecardresearch.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp9);
	let resp10 = match reqwest::blocking::get("*://*.zedo.com/*") {
		Ok(server) => server.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};
	println!("{}", resp10);
	/*
	let make_service = Shared::new(service_fn(log));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = Server::bind(&addr).serve(make_service);

	if let Err(e) = server.await {
		println!("error: {}", e);
	}
	*/
}

async fn handle(req: Request<Body>) -> hyper::Result<Response<Body>> {
	let client = Client::new();
	client.request(req).await
}

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

	let path = req.uri().path();

	if path.starts_with("*://*.doubleclick.net/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://partner.googleadservices.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.googlesyndication.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.google-analytics.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://creative.ak.fbcdn.net/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.adbrite.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.exponential.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.quantserve.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.scorecardresearch.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.zedo.com/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}

	if path.starts_with("*://*.httpbin.org/*") {
		println!("Site popup encotrado encontrado = {}", path);
	} else {
		println!("Generic path = {}", path)
	}
	handle(req).await
}