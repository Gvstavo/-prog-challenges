use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash)]
enum Hand{
	Paper,
	Rock,
	Scissors,
	Lizard,
	Spock
}

#[derive(Debug, Clone)]
enum Winner{
	Draw,
	Player1(Hand),
	Player2(Hand)
}

impl PartialEq for Winner {
	fn eq(&self, other: &Self) -> bool {
  	match (self.clone()  ,other.clone()) {
  		(Winner::Draw, Winner::Draw) => true,
  		(Winner::Player1(h1), Winner::Player1(h2))  if h1 == h2 => true,
  		(Winner::Player2(h1), Winner::Player2(h2))  if h1 == h2 => true,
  		_ => false,
  	}   
  }
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
	fn winner(&self, h1: Hand , h2: Hand) -> Winner{
		
		if h1 == h2{
			return Winner::Draw;
		};

		let v = self.hand.get(&h1).unwrap();

		match v.iter().find(|&x| *x == h2) {

			Some(_) => Winner::Player1(h1),
			_ => Winner::Player2(h2)
		}

	}

}

fn main() {
  let a = Rule::new();

  assert_eq!(a.winner(Hand::Paper, Hand::Paper), Winner::Draw);
  assert_eq!(a.winner(Hand::Paper, Hand::Rock), Winner::Player1(Hand::Paper));
  assert_eq!(a.winner(Hand::Paper, Hand::Spock), Winner::Player1(Hand::Paper));
  assert_eq!(a.winner(Hand::Paper, Hand::Scissors), Winner::Player2(Hand::Scissors));
  assert_eq!(a.winner(Hand::Paper, Hand::Lizard), Winner::Player2(Hand::Lizard));

  assert_eq!(a.winner(Hand::Rock, Hand::Rock), Winner::Draw);
  assert_eq!(a.winner(Hand::Rock, Hand::Scissors), Winner::Player1(Hand::Rock));
  assert_eq!(a.winner(Hand::Rock, Hand::Lizard), Winner::Player1(Hand::Rock));
  assert_eq!(a.winner(Hand::Rock, Hand::Paper), Winner::Player2(Hand::Paper));
  assert_eq!(a.winner(Hand::Rock, Hand::Spock), Winner::Player2(Hand::Spock));

  assert_eq!(a.winner(Hand::Scissors, Hand::Scissors), Winner::Draw);
  assert_eq!(a.winner(Hand::Scissors, Hand::Paper), Winner::Player1(Hand::Scissors));
  assert_eq!(a.winner(Hand::Scissors, Hand::Lizard), Winner::Player1(Hand::Scissors));
  assert_eq!(a.winner(Hand::Scissors, Hand::Spock), Winner::Player2(Hand::Spock));
  assert_eq!(a.winner(Hand::Scissors, Hand::Rock), Winner::Player2(Hand::Rock));


  assert_eq!(a.winner(Hand::Lizard, Hand::Lizard), Winner::Draw);
  assert_eq!(a.winner(Hand::Lizard, Hand::Spock), Winner::Player1(Hand::Lizard));
  assert_eq!(a.winner(Hand::Lizard, Hand::Paper), Winner::Player1(Hand::Lizard));
  assert_eq!(a.winner(Hand::Lizard, Hand::Rock), Winner::Player2(Hand::Rock));
  assert_eq!(a.winner(Hand::Lizard, Hand::Scissors), Winner::Player2(Hand::Scissors));


  assert_eq!(a.winner(Hand::Spock, Hand::Spock), Winner::Draw);
  assert_eq!(a.winner(Hand::Spock, Hand::Rock), Winner::Player1(Hand::Spock));
  assert_eq!(a.winner(Hand::Spock, Hand::Scissors), Winner::Player1(Hand::Spock));
  assert_eq!(a.winner(Hand::Spock, Hand::Lizard), Winner::Player2(Hand::Lizard));
  assert_eq!(a.winner(Hand::Spock, Hand::Paper), Winner::Player2(Hand::Paper));

}
