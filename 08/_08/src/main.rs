mod hang;
#[macro_use]
extern crate actix_web;
use actix_web::{web, App , HttpResponse, HttpServer, Error, HttpRequest};
use actix_files::Files;
use actix_web_actors::ws;
use hang::Hang;


async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  println!("{:?}", r);
  let res = ws::start(Hang::new(), &r, stream);
  println!("{:?}", res);
  res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  println!("Listening on port 8080");
  HttpServer::new(|| {
    App::new()
    .service(web::resource("/ws/").route(web::get().to(ws_index)))
    .service(Files::new("/","static").index_file("index.html"))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
    
}
