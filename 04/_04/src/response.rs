use serde::{Deserialize, Serialize};
use openssl::rsa::*;
#[derive(Deserialize, Serialize, Debug)]
pub struct Response{
	pub private_key: String,
	pub public_key: String
}


impl Response{

	pub fn new(n: u32) -> Self {

		let rsa = Rsa::generate(n).unwrap();

		let public_key = rsa.public_key_to_pem().unwrap();

		let private_key = rsa.private_key_to_pem().unwrap();

		Response{
			public_key: String::from_utf8(public_key).unwrap(),
			private_key: String::from_utf8(private_key).unwrap(),
		}
	}
}
