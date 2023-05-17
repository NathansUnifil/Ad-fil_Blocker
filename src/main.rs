use std::net::SocketAddr;
use hyper::{service::service_fn, Body, Client, Request, Response, Server};
use reqwest;
use tower::make::Shared;
//use anchor_lang::solana_program::entrypoint::ProgramResult;


#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>>{
	loop {
		// let filtros = ["*://*.doubleclick.net/*",
		//	"*://partner.googleadservices.com/*",
		//	"*://*.googlesyndication.com/*",
		//	"*://*.google-analytics.com/*",
		//	"*://creative.ak.fbcdn.net/*",
		//	"*://*.adbrite.com/*",
		//	"*://*.exponential.com/*",
		//	"*://*.quantserve.com/*",
		//	"*://*.scorecardresearch.com/*",
		//	"*://*.zedo.com/*", ];

		// new client

		let client = hyper::Client::new();

		// Bob the builder for getting the url
		let req1 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.doubleclick.net/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		// Pass our request builder object to our client.
		let hyperresp1 = client.request(req1).await?;

		// Get the response body bytes.
		let body_bytes1 = hyper::body::to_bytes(hyperresp1.into_body()).await?;

		// 2

		let body1 = String::from_utf8(body_bytes1.to_vec()).unwrap();

		let req2 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://partner.googleadservices.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp2 = client.request(req2).await?;

		let body_bytes2 = hyper::body::to_bytes(hyperresp2.into_body()).await?;

		let body2 = String::from_utf8(body_bytes2.to_vec()).unwrap();

		// 3

		let req3 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.googlesyndication.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp3 = client.request(req3).await?;

		let body_bytes3 = hyper::body::to_bytes(hyperresp3.into_body()).await?;

		let body3 = String::from_utf8(body_bytes3.to_vec()).unwrap();

		// 4

		let req4 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.google-analytics.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp4 = client.request(req4).await?;

		let body_bytes4 = hyper::body::to_bytes(hyperresp4.into_body()).await?;

		let body4 = String::from_utf8(body_bytes4.to_vec()).unwrap();

		// 5

		let req5 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://creative.ak.fbcdn.net/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp5 = client.request(req5).await?;

		let body_bytes5 = hyper::body::to_bytes(hyperresp5.into_body()).await?;

		let body5 = String::from_utf8(body_bytes5.to_vec()).unwrap();

		// 6

		let req6 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.adbrite.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp6 = client.request(req6).await?;

		let body_bytes6 = hyper::body::to_bytes(hyperresp6.into_body()).await?;

		let body6 = String::from_utf8(body_bytes6.to_vec()).unwrap();

		// 7

		let req7 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.exponential.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp7 = client.request(req7).await?;

		let body_bytes7 = hyper::body::to_bytes(hyperresp7.into_body()).await?;

		let body7 = String::from_utf8(body_bytes7.to_vec()).unwrap();

		// 8

		let req8 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.quantserve.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp8 = client.request(req8).await?;

		let body_bytes8 = hyper::body::to_bytes(hyperresp8.into_body()).await?;

		let body8 = String::from_utf8(body_bytes8.to_vec()).unwrap();

		// 9

		let req9 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.scorecardresearch.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp9 = client.request(req9).await?;

		let body_bytes9 = hyper::body::to_bytes(hyperresp9.into_body()).await?;

		let body9 = String::from_utf8(body_bytes9.to_vec()).unwrap();

		// 10

		let req10 = hyper::Request::builder()
			.method(hyper::Method::GET)
			.uri("*://*.zedo.com/*")
			.header("user-agent", "the-awesome-agent/007")
			.body(hyper::Body::from(""))?;

		let hyperresp10 = client.request(req10).await?;

		let body_bytes10 = hyper::body::to_bytes(hyperresp10.into_body()).await?;

		let body10 = String::from_utf8(body_bytes10.to_vec()).unwrap();

		// Responses start here

		let resp1 = match reqwest::blocking::get(body1) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp1);
		let resp2 = match reqwest::blocking::get(body2) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp2);
		let resp3 = match reqwest::blocking::get(body3) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp3);
		let resp4 = match reqwest::blocking::get(body4) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp4);
		let resp5 = match reqwest::blocking::get(body5) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp5);
		let resp6 = match reqwest::blocking::get(body6) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp6);
		let resp7 = match reqwest::blocking::get(body7) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp7);
		let resp8 = match reqwest::blocking::get(body8) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp8);
		let resp9 = match reqwest::blocking::get(body9) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp9);
		let resp10 = match reqwest::blocking::get(body10) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error: {}", err)
		};
		println!("{}", resp10);

	let make_service = Shared::new(service_fn(log));
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let server = Server::bind(&addr).serve(make_service);

	if let Err(e) = server.await {
		println!("error: {}", e);
	}
		// Ok::<(), e>(()).expect("TODO: panic message");
	}
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