use serde::{Serialize , Deserialize};

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
