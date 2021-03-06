use serde::{Serialize , Deserialize};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use reqwest::*;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use chrono::{Duration, Utc};


#[derive(Debug , Serialize , Deserialize)]
pub struct Response{
	pub nome: String, 
	pub idade: i64,
	pub cpf: String,
	pub data_nasc: String,
	pub cep: String,
	pub endereco: String,
	pub bairro: String,
	pub cidade: String,
	pub estado: String,
	pub altura: f64,
	pub peso: f64,
	pub tipo_sanguineo: String,
}

pub fn cpf() -> String {

	let mut rng = thread_rng();

	let mut count = 1;

	let mut v1 = 0;
	let mut v2 = 0;


	let mut v : Vec<u32>= (0..=8).map(|x| rng.gen_range(0..=9)).collect();

	for i in &v {
	
		v1 = v1 + i * count;
		count +=1;

	}

	v1 = (v1 % 11) % 10;

	count = 0;

	v.push(v1);

	for i in &v {
	
		v2 = v2 + i * count;

		count +=1;

	}

	v2 = (v2 % 11) % 10;

	v.push(v2);
	v.iter().fold(String::new(), |acc, x| format!("{}{}",acc,x))

}

fn nome() -> String{

	let mut rng = thread_rng();
	
	let mut file = File::open("nomes.txt").unwrap();

	let mut buf_reader = BufReader::new(file);
  	
  let mut contents = String::new();

  buf_reader.read_to_string(&mut contents).unwrap();  

  let n: usize = rng.gen_range(0..2823);

	contents.split("\n").collect::<Vec<&str>>().get(n).unwrap().to_string()

}

pub fn sobrenome() -> String{

	let mut rng = thread_rng();
	
	let mut file = File::open("sobrenomes.txt").unwrap();

	let mut buf_reader = BufReader::new(file);
  	
  let mut contents = String::new();

  buf_reader.read_to_string(&mut contents).unwrap();  

  let n: usize = rng.gen_range(0..254);

	contents.split("\n").collect::<Vec<&str>>().get(n).unwrap().to_string()

}

impl Response{

	pub fn new() -> Self{

		let sang : Vec<&str> = vec!["A+", "B+", "O+", "AB+", "A-", "B-", "O-"  , "AB-" ];

		let mut rng = thread_rng();

		let days : i64 = rng.gen_range(6570..=36500);

		let nascimento =  Utc::today() - Duration::days(days);

		let idade : i64 = days / 365 ;

		let mut file = File::open("ceps.txt").unwrap();

		let peso  : f64 = rng.gen_range(55.0..100.0);

  	let mut buf_reader = BufReader::new(file);
  	
  	let mut contents = String::new();

  	buf_reader.read_to_string(&mut contents).unwrap();  

  	let n: usize = rng.gen_range(0..732762);

  	let u: usize = rng.gen_range(0..8);

  	let cep  = contents.split("\n").map(|x| x.get(..8)).collect::<Vec<Option<&str>>>().get(n).unwrap().unwrap();

  	let url : String = format!("http://cep.republicavirtual.com.br/web_cep.php?formato=json&cep={}",cep); 

  	let resp = reqwest::blocking::get(&url).unwrap()
        .json::<HashMap<String, String>>().unwrap();

    Response{
    	nome: format!("{} {}", nome(), sobrenome()),
    	idade : idade,
    	cpf: cpf(),
    	data_nasc: nascimento.to_string(),
    	cep: cep.to_string(),
    	endereco: format!("{} {}", resp.get("tipo_logradouro").unwrap(), resp.get("logradouro").unwrap()),
    	cidade: resp.get("cidade").unwrap().to_owned(),
    	bairro: resp.get("bairro").unwrap().to_owned(),
    	estado: resp.get("uf").unwrap().to_owned(),
    	altura: rng.gen_range(1.6..2.0), 
    	peso: peso,
    	tipo_sanguineo: sang.get(u).unwrap().to_string()

    }

	}
}


#[test]
fn pessoa(){
	println!("{:?}",Response::new());
}


