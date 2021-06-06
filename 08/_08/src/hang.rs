use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use actix::{Context};
use actix::prelude::*;
use actix_web_actors::ws;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
pub struct Hang{
	word: String
}

impl Hang{
	pub fn new() -> Self{
		let mut rng = rand::thread_rng();
		let line = rng.gen_range(0..=370103) as usize;

		let word = read_lines("words.txt").unwrap().nth(line).unwrap().unwrap().to_string();

		Hang{
			word: word
		}
	}
}


impl Actor for Hang {
	type Context = ws::WebsocketContext<Self>;
	fn started(&mut self, ctx: &mut Self::Context) {
  	ctx.text(&self.word)
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Hang {
  fn handle(&mut self,msg: Result<ws::Message, ws::ProtocolError>,ctx: &mut Self::Context) {
		println!("WS: {:?}", msg);
		match msg {
		  Ok(ws::Message::Text(text)) => ctx.text("^".to_string()),
		  Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
		  Ok(ws::Message::Close(reason)) => {
		  	ctx.close(reason);
		  	ctx.stop();
		  }
		    _ => ctx.stop(),
		}
  }
}


