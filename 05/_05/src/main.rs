fn main() {


	let test = 	|x : i32| -> (){
		
		match x {

			_ if x % 3 == 0 && x % 5 == 0 => println!("FizzBuzz"),
			_ if x % 3 == 0 => println!("Fizz"),
			_ if x % 5 == 0 => println!("Buzz"),
			_ => println!("{:?}",x)

		}

	};

	(1..=100).for_each(test);

}
