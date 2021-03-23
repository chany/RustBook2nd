use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(5);  // MutexGuard 스마트포인터 반환(Mutex로 보호되고 값 5를 가짐)

    {
        let mut num = m.lock().unwrap();    // lock을 얻고 m 내부값에 대한 가변 참조자
        *num = 6;
    }       // 자동 unlock

    println!("m = {:?}", m);    // 6

    //--------------

    let counter = Mutex::new(0);        // Mutex로 보호되고 값 0을 갖는 counter 변수
    let mut handles = vec![];

    for _ in 0..10 {                    // 스레드 10개 생성
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // lock을 얻고, counter가리키는 참조자 반환
            *num += 1;                              // 참조자가 가리키는 counter값을  lock 얻은 이 스레드가 변경(다른 스레드는 접근 불가)
        });                                         // lock 해제
        handles.push(handle);                       // 벡터에 넣음
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    

}