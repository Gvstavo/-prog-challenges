use serde::{Serialize , Deserialize};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug , Serialize , Deserialize)]
pub struct Response{

	pub nome: String, 
	pub idade: u32,
	pub cpf: String,
	pub data_nasc: String,
	pub sexo: String,
	pub signo: String,
 	pub mae: String,
	pub pai: String,
	pub email: String,
	pub senha: String,
	pub cep: String,
	pub endereco: String,
	pub numero: u32,
	pub bairro: i32,
	pub cidade: String,
	pub estado: String,
	pub telefone_fixo: String,
	pub celular: String,
	pub altura: String,
	pub peso: String,
	pub tipo_sanguineo: String,
	pub cor: String
}

#[test]

fn file(){

	let mut file = File::open("ceps.txt").unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();

	println!("{:?}",contents.split("\n"));
}