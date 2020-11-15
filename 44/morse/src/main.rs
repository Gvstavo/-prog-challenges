mod morse;

mod parser;

use morse::Morse;

fn main() {

  	
  	let mut m = Morse::new_plain("bom dia");

  	m.encode().unwrap();
  	m.emit();



	

}
