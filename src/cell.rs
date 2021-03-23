use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
	let cell = Cell::new(1234);

	cell.set(4321);
	let value = cell.into_inner();

	println!("{}", value);

	// RefCell
	let cell = RefCell::new(0);
	*cell.borrow_mut() += 10;
	*cell.borrow_mut() += 20;

	let value = cell.into_inner();

	println!("{}", value);
}
