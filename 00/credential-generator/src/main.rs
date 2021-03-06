#[macro_use]
extern crate actix_web;

use actix_files::Files;
use actix_web::{http, web, App ,  HttpServer, Responder, HttpResponse, Error};

mod response;
use response::Response;

// #[get("/new")]
#[get("/")]
async fn generator()-> impl Responder {
	HttpResponse::Ok().json(Response::new())
}

// #[get("/")]
// async fn index() -> impl Responder {	

// 	HttpResponse::build(http::StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../static/index.html"))
// }

#[actix_web::main]
async fn main()  -> std::io::Result<()> {

	println!("Listening on port 8080");

	HttpServer::new(move ||  {

		App::new()
	//	.service(index)
		.service(generator)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}



