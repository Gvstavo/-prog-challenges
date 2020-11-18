use std::fs::File;
use rodio::Sink;
use std::io::BufReader;	
use super::parser::Parser;

#[derive(Debug)]

pub struct Morse{

	plain: String,
	encoded: String
}

impl Morse{

	pub fn new() -> Self {

		Morse{
			plain:  String::new(),
			encoded:  String::new()
		}

	}

	pub fn new_plain(msg: &str) -> Self {

		Morse{
			
			plain: msg.to_string(),
			encoded:  String::new()
		
		}


	}

	pub fn new_encoded(msg: &str) -> Self {

		Morse{
			
			plain: String::new(),
			encoded:  msg.to_string()
		
		}


	}

	pub fn plain(&self) -> &str {
		&self.plain
	}
	
	pub fn coded(&self) -> &str {
		&self.encoded
	}	
	pub fn encode(&mut self) -> Result<(), ()> {

		let p = Parser::new();

		for i in self.plain.chars(){
				
			match p.search(i){

				Some(value) => self.encoded.push_str(format!("{} ",value).as_str()),
				_ => self.encoded.push_str("#")

			}
		};

		Ok(())
		
	}

	pub fn decode(&mut self) -> Result<() , ()>{
		
		let p = Parser::new();

		let v = self.encoded.split(" ").collect::<Vec<&str>>();

		for i in v {

			match p.search_plain(i) {

				Some(value) =>  self.plain.push_str(format!("{}",value).as_str()),
				_ => continue,
			}

		}

		Ok(())
	}


	pub fn emit(&mut self) ->(){

		if self.encoded.is_empty(){
			return ()
		}


		let sound = |f:&str| -> (){
			let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
			let sink = rodio::Sink::try_new(&stream_handle).unwrap();

			let file = File::open(f).unwrap();
	 		let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

			sink.append(source);   	
			loop{ if sink.empty(){ break;}}

		};

		let dd : Vec<String> = self.encoded
																			.split("")
																			.collect::<Vec<&str>>()
																			.iter()
																			.filter(|x| **x == "." || **x == "-")
																			.collect::<Vec<&&str>>()
																			.iter()
																			.map(|x| x.to_string())
																			.collect::<Vec<String>>();

		
		for i in dd {

			let filename = match i.as_str() {

				"-" =>  "T_morse_code.ogg",
				_ =>  "E_morse_code.ogg"
			};


			sound(filename);

		}																	


	}	

}