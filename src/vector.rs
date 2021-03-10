fn main() {
  let v = vec![1,2,3,4,5];

  let third: &i32 = &v[2];
  let does_not_exist = &v[100];
  println!("세번째 원소 {}", third);
  println!("101번째 원소 {}", does_not_exist);

  match v.get(2) {
	  Some(third) => println!("세번째 원소는 {}", third),
	  None => println!("세번째 원소 없음"),
  }
}
