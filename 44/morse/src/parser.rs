use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;

#[derive(Debug)]

pub struct Parser{

	code: 	HashMap<String, String>,
	plain:  HashMap<String, String>

}

impl Parser{
	

	pub fn new() -> Self {


		let mut f = File::open("code.txt").unwrap();

		let mut buffer = String::new();

		let mut coded : HashMap<String, String> = HashMap::new(); 

		let mut plain : HashMap<String, String> = HashMap::new(); 

		f.read_to_string(&mut buffer).unwrap();


		let  k_v : Vec<String> = buffer
																									.split("\n")
																									.collect::<Vec<&str>>()
																									.iter()
																									.map(|x| x.to_string())
																									.collect::<Vec<String>>();

		
		for i in k_v{

			let aux : Vec<String> = i.split(" ")
																						.collect::<Vec<&str>>()
																						.iter()
																						.map(|x| x.to_string())
																						.collect::<Vec<String>>();


			coded.insert(aux.get(0) .unwrap().to_string(), aux.get(1).unwrap().to_string());																			

			plain.insert(aux.get(1) .unwrap().to_string(), aux.get(0).unwrap().to_string());

		}	;

		coded.insert(" ".to_string() , "/".to_string());
		plain.insert("/".to_string() , " ".to_string());

		Parser{
			code: coded , 
			plain: plain
		}

	}	

	pub fn search_plain(&self , c:  &str) -> Option<&String>{

		let mut src = c.to_string();

		src.make_ascii_uppercase();

		self.plain.get(&src)


	}

	pub fn search(&self , c: char) -> Option<&String> { 

		let mut src = c.to_string();

		src.make_ascii_uppercase();

		self.code.get(&src)

	}

}