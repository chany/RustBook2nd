struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("data `{}`를 갖는 CustomSmartPointer 버리기!", self.data);
	}
}

fn main() {
	let c = CustomSmartPointer { data: String::from("my stuff") };
	let d = CustomSmartPointer { data: String::from("other stuff") };
	println!("CustomSmartPointer 생성.");
}
