
fn collatz(n: i64) -> Vec<i64>{

	let mut ret : Vec<i64> = vec![];
	let mut aux = n;
	
	while aux > 1{

		match aux % 2{

			0  => aux/=2 ,
			_ =>  aux = 3 * aux + 1

		};

		ret.push(aux);

	}


	ret 


}



fn main(){


	println!("{:?}",collatz(12) );
		




}  