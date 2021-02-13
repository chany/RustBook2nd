fn main() {
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if counter == 10 {
			break counter * 2;
		}
	};

	println!("counter={}, result={}", counter, result);


	let mut x = 0;
	while x < 10 {      // x가 10보다 작은 동안
		x += 1;
		println!("x={}", x);
	}

	let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	for x in xs.iter() {
		println!("x={}", x);
	}
}
