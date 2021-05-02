use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request{
	pub n: u32
}