fn fibonacci(depth: u64) {
	let mut old : u64 = 0;  // fibonacci(i - 1) - value of previous iteration
	let mut older: u64 = 0; // fibonacci(i - 2) - value of pre-previous iteration

	for i in 0..depth  {

		if i == 0 || i == 1 {
			older = old;
			old = i;
			println!("Fib({}) = {}",i, i);
		} else {
			let fibonaci_number: u64 = old + older;
			println!("Fib({}) = {}",i, fibonaci_number);
			older = old;
			old = fibonaci_number;
		}
	}
}

fn main() {
	fibonacci(20);
}