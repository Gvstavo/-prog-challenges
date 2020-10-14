


fn factorial(n: u64) -> u64{


	(1..=n).product()
}


fn main() {
	
	let n :	u64 = 9;
	println!("{:?}", factorial(n));

}