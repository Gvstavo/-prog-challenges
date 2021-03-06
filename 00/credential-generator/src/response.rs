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
	// dados pessoais
	pub nome: String, 
	pub idade: u32,
	pub cpf: String,
	pub data_nasc: String,
	pub sexo: String,
	pub signo: String,
	// filiacao
 	pub mae: String,
	pub pai: String,
	// endereço
	pub cep: String,
	pub endereco: String,
	pub numero: u32,
	pub bairro: i32,
	pub cidade: String,
	pub estado: String,
	// telefpones
	pub telefone_fixo: String,
	pub celular: String,
	// caracteristicas fisicas
	pub altura: f64,
	pub peso: String,
	pub tipo_sanguineo: String,
}

pub fn cpf() -> String {

	let mut rng = thread_rng();

	let mut count = 0;

	let mut v1 = 0;
	let mut v2 = 0;

	let mut v : Vec<u32>= (0..=8).map(|x| rng.gen_range(0..=9)).collect();

	for i in &v {

		v1 = v1 + i * (9 - (count % 10));

		v2 = v2 + i * (9 - ((count + 1) % 10));

		count +=1;

	}

	v1 = (v1 % 11) % 10;
	v2 = v2  + v1 * 9;
	v2 = (v2 % 11) % 10;

	v.push(v1);
	v.push(v2);

	v.iter().fold(String::new(), |acc, x| format!("{}{}",acc,x))

}

impl Response{

	pub fn new() -> (){

		let sang : Vec<&str> = vec!["A+", "B+", "O+", "AB+", "A-", "B-", "O-"  , "AB-" ];

		let mut rng = thread_rng();

		let days : i64 = rng.gen_range(6570..=36500);

		let nascimento =  Utc::today() - Duration::days(days);

		let mut file = File::open("ceps.txt").unwrap();

		let peso  : f64 = rng.gen_range(55.0..100.0);

  	let mut buf_reader = BufReader::new(file);
  	
  	let mut contents = String::new();

  	buf_reader.read_to_string(&mut contents).unwrap();  

  	let n: usize = rng.gen_range(0..732762);

  	let cep  = contents.split("\n").map(|x| x.get(..8)).collect::<Vec<Option<&str>>>().get(n).unwrap().unwrap();


  	let url : String = format!("http://cep.republicavirtual.com.br/web_cep.php?formato=json&cep={}",cep); 

  	let resp = reqwest::blocking::get(&url).unwrap()
        .json::<HashMap<String, String>>().unwrap();

	}
}

#[test]
fn cpf_test(){

	let mut rng = thread_rng();

	let mut count = 0;

	let mut v1 = 0;
	let mut v2 = 0;

	let mut v  = [1,2,3,4,5,6,7,8,9].to_vec();
//	let mut v : Vec<u32>= (0..=8).map(|x| rng.gen_range(0..=9)).collect();

	for i in &v {

		v1 = v1 + i * (9 - (count % 10));

		v2 = v2 + i * (9 - ((count + 1) % 10));

		count +=1;

	}

	v1 = (v1 % 11) % 10;
	v2 = v2  + v1 * 9;
	v2 = (v2 % 11) % 10;

	v.push(v1);
	v.push(v2);

//	let ret = v.iter().fold(String::new(), |acc, x| format!("{}{}",acc,x));
	println!("{:?} {:?}",v1,v2 );
}
//#[test]

// fn request(){
  
//   let resp = reqwest::blocking::get("http://cep.republicavirtual.com.br/web_cep.php?formato=json&cep=99714413").unwrap()
//         .json::<HashMap<String, String>>().unwrap();
//   println!("{:#?}", resp);

// }

// fn float_random(){

// 	let mut rng = thread_rng();

// 	let v : Vec<u32>= (0..=9).map(|x| rng.gen_range(0..=9)).collect();

// 	for i in &v {
// 		println!("{:?}",i * 2);
// 	}

// 	println!("{:?}", v);

// }
// fn file(){

// 	let mut file = File::open("ceps.txt").unwrap();

//   let mut buf_reader = BufReader::new(file);
//   let mut contents = String::new();
//   buf_reader.read_to_string(&mut contents).unwrap();

//   let mut rng = thread_rng();

//   let n: usize = rng.gen_range(0..732762);

// 	println!("{:?}",contents.split("\n").map(|x| x.get(..8)).collect::<Vec<Option<&str>>>().get(n));
// }

