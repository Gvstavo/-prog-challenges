use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Encrypt{
	pub public_key: String,
	pub plain_text: String
}
#[derive(Deserialize, Serialize)]
pub struct ResponseEncrypt{
	pub text: String
}