fn main() {
  let s1 = String::from("Hello");
  let s2 = s1;      // s1은 이후 더이상 사용될 수 없음
  // println!("s1={}", s1);

  //----
  let s1 = String::from("Hello");
  let s2 = s1.clone();          // deep copy
  println!("s1={}, s2={}", s1, s2);

  //----
  let s = String::from("hello");
  takes_ownership(s);   // s의 소유권이 함수 내부로 넘어가고 이후 s 사용 불가
  let x = 5;
  makes_copy(x);        // Copy trait이 구현된 정수형은 소유권 복사

  //----
  let s1 = gives_ownership();
  let s2 = String::from("hello for s2");
  let s3 = takes_and_gives_back(s2);    // s2는 이후 사용될 수 없음
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} 

fn gives_ownership() -> String {
    let some_string = String::from("hello for s1");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}