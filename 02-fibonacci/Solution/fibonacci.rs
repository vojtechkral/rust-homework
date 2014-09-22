fn main() {
	fibonacci(20i);
}

fn fibonacci(depth: int) {
	let mut old : u64 = 0; //fibonacci(i - 1) - value of previous iteration
	let mut older: u64 = 0; //fibonacci(i - 2) -value of pre-previous iteration

	for i in range(0u64,depth as u64) {

		if i == 0u64 || i == 1u64 {
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
