use std::error::Error;
use reqwest;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderName};
use reqwest::Method;

mod cookie {

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

fn bloquear_cookies(arr: &[&str]) -> Result<(), Box<dyn Error>> {
    let resposta_cookie = arr.text()?;
    println!("arr = {:?}", resposta_cookie);
}

}