// 사용자 입력을 받고, 해당 입력값을 숫자로 변환
// 랜덤값과 비교하여 더 작은지, 큰지 출력하고 맞출 때까지 반복
extern crate rand;

use std::io;        // 사용자 키보드 입력을 위한 stdio() 사용하기 위해 준비
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자 맞추기 게임!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("비밀 숫자는 {}", secret_number);

    loop {
        println!("추측하는 숫자를 입력하시오.");

        let mut guess = String::new();      // 사용자 입력값 저장을 위한 새로운 메모리 공간 할당

        io::stdin().read_line(&mut guess)   // guess에 대한 가변참조를 함수에게 전달
                .expect("입력 실패");
        // read_line()은 매개변수에 사용자 입력을 저장하고, io::Result 타입의 값 반환(Ok 또는 Err)
        // 반환값이 Err인 경우, io::Result가 가진 메소드(함수)가 프로그램을 중지하고 expect() 매개변수 출력
        // 반환값이 Ok인 경우, io::Result가 가진 메소드(함수)가 사용자가 입력한 값의 바이트 수를 반환

        let guess: u32 = match guess.trim().parse() {      // guess shadowing하며 숫자로 변환
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("추측한 값은 {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("너무 작음!"),
            Ordering::Greater => println!("너무 큼!"),
            Ordering::Equal   => {
                println!("맞았음!");
                break;
            }
        }
    }
}
