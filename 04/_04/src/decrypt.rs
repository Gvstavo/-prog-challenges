use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Decrypt{
	pub private_key: String,
	pub encrypted_text: String
}
#[derive(Deserialize, Serialize)]
pub struct ResponseDecrypt{
	pub text: String
}