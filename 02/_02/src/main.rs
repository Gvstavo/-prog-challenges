#[macro_use]
extern crate actix_web;


use actix_web::{web, App , HttpResponse, HttpServer, Responder};
use actix_files::Files;
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	println!("Listening on port 8080");
	HttpServer::new(|| {
		App::new()
		//.route("/", web::get().to(hello))
		.service(Files::new("/","static").index_file("index.html"))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
