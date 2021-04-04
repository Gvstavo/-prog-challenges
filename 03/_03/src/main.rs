#[macro_use]
extern crate actix_web;

use actix_web::{web, App , HttpResponse, HttpServer};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime,Utc,DateTime};


#[derive(Deserialize, Serialize)]
struct Request{
	date: String,
	time: String
}

#[derive(Deserialize, Serialize)]
struct Response{
	value: String
}


#[post("/to")]
async fn conversor(t: web::Json<Request>)->HttpResponse {

	let from_str = format!("{} {}", t.date , t.time);

	println!("{}", from_str);

	let naive = NaiveDateTime::parse_from_str(from_str.as_str() , "%Y-%m-%d %H:%M").unwrap();

	let dt = DateTime::<Utc>::from_utc(naive, Utc);
	
	let ret = Utc::now() - dt;

	HttpResponse::Ok().json(Response{value: ret.num_seconds().to_string()})

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

