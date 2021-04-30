#[macro_use]
extern crate actix_web;

use actix_web::{web, App , HttpResponse, HttpServer};
use actix_files::Files;
use serde::{Deserialize, Serialize};

mod response;
use response::Response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	println!("Listening on port 8080");
	HttpServer::new(|| {
		App::new()
		.service(Files::new("/","static").index_file("index.html"))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

