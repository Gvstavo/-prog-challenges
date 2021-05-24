use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash)]
enum Hand{
	Paper,
	Rock,
	Scissors,
	Lizard,
	Spock
}
impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
  	match (self  ,other) {
  		(&Hand::Paper, &Hand::Paper) => true,
  		(&Hand::Rock, &Hand::Rock) => true,
  		(&Hand::Scissors, &Hand::Scissors) => true,
  		(&Hand::Lizard, &Hand::Lizard) => true,
  		(&Hand::Spock, &Hand::Spock) => true,
  		_ => false,
  	}   
  }
}

#[derive(Debug, Clone)]
struct Rule{
	hand: HashMap<Hand, Vec<Hand>>
}

impl Rule{
	fn new() -> Self{
		let mut a = HashMap::new();

		a.insert(Hand::Paper , vec![Hand::Rock,Hand::Spock]);
		a.insert(Hand::Rock , vec![Hand::Lizard,Hand::Scissors]);
		a.insert(Hand::Scissors, vec![Hand::Paper , Hand::Lizard]);
		a.insert(Hand::Lizard, vec![Hand::Spock, Hand::Paper]);
		a.insert(Hand::Spock, vec![Hand::Scissors, Hand::Rock]);

		Rule{
			hand: a
		}
	}
	// fn winner(&self, h1: Hand , h2: Hand) -> Hand{
		
	// }

}

fn main() {
    println!("Hello, world!");
}
