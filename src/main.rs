use reqwest;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderName};
use reqwest::Method;

const  arr: &'static [&'static str]  = &["https://g1.globo.com/?utm_source=barraGCOM",
"https://www.theonion.com/",
];
const arr_size:i32 = 2;

fn achar_cookies() -> Result<(), Box<dyn std::error::Error>> {
	let COOKIE = assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
	adicionar_cookie_bloqueado(COOKIE)
}

fn adicionar_cookie_bloqueado(method: HeaderName) -> Result<(), Box<dyn std::error::Error>> {
	let resp1 = reqwest::get(b"GET");
	1 + arr_size;
	arr[arr_size] = &*String::from(resp1);
	bloquear_cookies(arr)
}

fn bloquear_cookies(arr: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
	let resposta_cookie = arr.text()?;
	println!("arr = {:?}", resposta_cookie);
}

fn todos_blockings_gets() ->  Result<(), Box<dyn std::error::Error>> {

	let resp1 = reqwest::blocking::get("https://www.youtube.com/")?;
	let body1 = resp1.text()?;
	println!("body1 = {:?}", body1);

	let resp2 = reqwest::blocking::get("https://www.climatempo.com/")?;
	let body2 = resp2.text()?;
	println!("body2 = {:?}", body2);

	let resp3 = reqwest::blocking::get("https://www.globo.com/")?;
	let body3 = resp3.text()?;
	println!("body3 = {:?}", body3);

	let resp4 = reqwest::blocking::get("https://brasilescola.uol.com.br/")?;
	let body4 = resp4.text()?;
	println!("body4 = {:?}", body4);

	let resp5 = reqwest::blocking::get("https://www.fifa.com/")?;
	let body5 = resp5.text()?;
	println!("body5 = {:?}", body5);

	let resp6 = reqwest::blocking::get("https://www.mercadolivre.com.br/")?;
	let body6 = resp6.text()?;
	println!("body6 = {:?}", body6);

	let resp7 = reqwest::blocking::get("https://www.olx.com.br/")?;
	let body7 = resp7.text()?;
	println!("body7 = {:?}", body7);



	Ok(())
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
	tokio::task::spawn_blocking(move || todos_blockings_gets().unwrap()).await?;
	// let filtros = ["https://doubleclick.net/",
	//	"https://googleadservices.com/",
	//	"https://googlesyndication.com/",
	//	"https://google-analytics.com/",
	//	"https://creative.ak.fbcdn.net/",
	//	"https://adbrite.com/",
	//	"https://exponential.com/",
	//	"https://quantserve.com/",
	//	"https://scorecardresearch.com/",
	//	"https://zedo.com/", ];
	Ok(())
}