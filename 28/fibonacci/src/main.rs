

fn phi() -> f64{

	(1.0_f64 + 5.0_f64.sqrt()) / 2.0_f64

}


fn fib(n: i64) -> i64{

	match n {

		0 | 1 => n,
		_ => fib(n - 1) + fib(n - 2)

	}
}


fn fib_by_golden_ratio(n : i32) -> f64{

	let ret = (phi().powi(n) / 5.0_f64.sqrt()) + 0.5_f64;

	ret.trunc()

}

fn main() {

	println!("{:?}", fib(10));
	println!("{:?}", fib_by_golden_ratio(10));

   
}