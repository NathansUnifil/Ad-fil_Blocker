
use reqwest;
use reqwest::blocking::Client;

fn todos_blockings_gets() ->  Result<(), Box<dyn std::error::Error>> {

	let resp1 = reqwest::blocking::get("https://www.youtube.com/")?;
	let body1 = resp1.text()?;
	println!("body1 = {:?}", body1);

	//Um teste aqui em baixo para testar as coisas

	/*

	let resp2 = match client.post( "https://www.youtube.com/").json(&map).send() {
		Ok(resp2) => resp2.text().unwrap(),
		Err(err) => panic!("Error: {}", err)
	};

	println!("{}", resp2);

	*/

	let resp2 = reqwest::blocking::get("https://www.facebook.com/")?;
	let body2 = resp2.text()?;
	println!("body2 = {:?}", body2);

	let resp3 = reqwest::blocking::get("https://www.twitter.com/")?;
	let body3 = resp3.text()?;
	println!("body3 = {:?}", body3);


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

	/*

    let resp1 = match reqwest::blocking::get("https://www.youtube.com/") {
        Ok(server) => server.text().unwrap(),
        Err(err) => panic!("Error 1 = {}", err)
    };
     */
	Ok(())
}