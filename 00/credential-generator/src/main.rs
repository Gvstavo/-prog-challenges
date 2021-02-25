use actix_files::Files;
use actix_web::{http, web, App ,  HttpServer, Responder, HttpResponse, Error};

mod response;
use response::Response;

async fn generator()-> Result<HttpResponse, Error> {

	Ok(HttpResponse::Ok().finish())


}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {

	println!("Listening on port 8080");

	HttpServer::new(move ||  {

		App::new()
		.service(Files::new("/","static").index_file("index.html"))
		.route("/new", web::get().to(generator))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await


}
