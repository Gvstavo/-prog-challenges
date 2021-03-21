#[macro_use]
extern crate actix_web;
use actix_web::{web, App , HttpResponse, HttpServer};
use actix_files::Files;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Request{
	x: f64,
	to: String,
	from: String
}


#[derive(Deserialize, Serialize)]
struct Response{
	value: f64
}




#[post("/to")]
async fn conversor(t: web::Json<Request>)->HttpResponse {
//	println!("^");
	let ret = match (t.to.as_str() , t.from.as_str()) {
		("c","f") => t.x * 9.0_f64 / 5.0_f64 + 32.0_f64,
		("c","k") => t.x + 273.15_f64,
		("f","c") => (t.x - 32.0_f64) * 5.0_f64/9.0_f64,
		("f","k") => (t.x - 32.0_f64) * 5.0_f64/9.0_f64 + 273.15_f64,
		("k","c") => t.x - 273.15_f64,
		("k","f") =>  (t.x - 273.15_f64 ) * 9.0_f64 / 5.0_f64 + 32.0_f64,
		(_ , _)  => t.x,
	};
	
	HttpResponse::Ok().json(Response{value: ret})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	println!("Listening on port 8080");
	HttpServer::new(|| {
		App::new()
		.service(conversor)
		.service(Files::new("/","static").index_file("index.html"))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
