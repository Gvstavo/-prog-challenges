#[macro_use]
extern crate actix_web;

use actix_web::{web, App , HttpResponse, HttpServer};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use openssl::rsa::*;

mod response;
use response::Response;

mod request;
use request::Request;

mod encrypt;
use encrypt::*;

mod decrypt;
use decrypt::*;

#[post("/keys")]
async fn key_pairs(t: web::Json<Request>)->HttpResponse {

	println!("generating keys...");
	let ret = Response::new(t.n);

	HttpResponse::Ok().json(ret)

}
#[post("/public_encrypt")]
async fn public_encrypt(t: web::Json<Encrypt>)->HttpResponse {

	println!("encrypting...");

	let u = t.public_key.as_bytes();

	let data = t.plain_text.as_bytes();
	println!("{:?}",data );
	let k = Rsa::public_key_from_pem(u).unwrap();

	let mut buffer : Vec<u8> = vec![0; k.size() as usize];
	k.public_encrypt(data, &mut buffer, Padding::PKCS1).unwrap();
			

	let ret = ResponseEncrypt{
		text: base64::encode(buffer)
	};

	HttpResponse::Ok().json(ret)


}

#[post("/private_decrypt")]
async fn private_decrypt(t: web::Json<Decrypt>)->HttpResponse {

	println!("decrypting...");

	let u = t.private_key.as_bytes();

	//let data = t.encrypted_text.as_bytes();
	let data = base64::decode(t.encrypted_text.clone()).unwrap();
	let k = Rsa::private_key_from_pem(u).unwrap();

	let mut buffer : Vec<u8> = vec![0; k.size() as usize];
	k.private_decrypt(&data, &mut buffer, Padding::PKCS1).unwrap();
			

	let ret = ResponseDecrypt{
		text: String::from_utf8_lossy(&buffer).to_string()
	};

	HttpResponse::Ok().json(ret)


}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	println!("Listening on port 8080");
	HttpServer::new(|| {
		App::new()
		.service(key_pairs)
		.service(public_encrypt)
		.service(private_decrypt)
		.service(Files::new("/","static").index_file("index.html"))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

