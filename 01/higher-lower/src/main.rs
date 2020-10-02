
use std::io::{self,   BufRead};
use rand::Rng;

fn main() {


	let stdin = io::stdin();
	let mut rng = rand::thread_rng();
	let number = rng.gen_range(0 , 100); 

	let mut tries = 1;

	println!("Write a number between [0 , 100]");



	for line in stdin.lock().lines() {
	  
	  match line.unwrap().parse::<u32>() {

	  	Ok(guess) if guess > number => println!("Larger"),
	  	Ok(guess) if guess < number => println!("Smaller"),
	  	Ok(guess) if guess > 100 => println!("Out of range!!"),
	  	Ok(guess) if guess == number =>{

	  		println!("Correct!");
	  		break;
	  	},	


	  	_ =>{ 
	  		println!("Error");
	  		break
	  	}	



	  }; 
	  tries = tries + 1; 


	}


	println!("Correct after {:?} tries", tries);

    
}
