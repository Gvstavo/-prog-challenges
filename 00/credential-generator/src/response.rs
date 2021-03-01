use serde::{Serialize , Deserialize};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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
	pub altura: String,
	pub peso: String,
	pub tipo_sanguineo: String,
}

#[test]

fn file(){

	let mut file = File::open("ceps.txt").unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();

	println!("{:?}",contents.split("\n").map(|x| x.get(..8)).collect::<Vec<Option<&str>>>());
}