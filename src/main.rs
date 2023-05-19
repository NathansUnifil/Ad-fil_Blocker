use reqwest;

fn main() ->  Result<(), Box<dyn std::error::Error>>{
	loop {
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

		let resp1 = match reqwest::blocking::get("https://www.youtube.com/") {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 1 = {}", err)
		};
		println!("{}", resp1);
		/*
		let resp2 = match reqwest::blocking::get(body2) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 2 = {}", err)
		};
		println!("{}", resp2);
		let resp3 = match reqwest::blocking::get(body3) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 3 = {}", err)
		};
		println!("{}", resp3);
		let resp4 = match reqwest::blocking::get(body4) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 4 = {}", err)
		};
		println!("{}", resp4);
		let resp5 = match reqwest::blocking::get(body5) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 5 = {}", err)
		};
		println!("{}", resp5);
		let resp6 = match reqwest::blocking::get(body6) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 6 = {}", err)
		};
		println!("{}", resp6);
		let resp7 = match reqwest::blocking::get(body7) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 7 = {}", err)
		};
		println!("{}", resp7);
		let resp8 = match reqwest::blocking::get(body8) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 8 = {}", err)
		};
		println!("{}", resp8);
		let resp9 = match reqwest::blocking::get(body9) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 9 = {}", err)
		};
		println!("{}", resp9);
		let resp10 = match reqwest::blocking::get(body10) {
			Ok(server) => server.text().unwrap(),
			Err(err) => panic!("Error 10 = {}", err)
		};
		println!("{}", resp10);
		 */
		return Ok(());
	}
}