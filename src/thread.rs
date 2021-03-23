use std::thread;
use std::time::Duration;

fn main() {
    {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {    // move없으면 다른 스레드에서 v 사용하려다가 에러
        println!("Here's a vector: {:?}", v);   // 여기서 v borrow
    });

    //drop(v);

    handle.join().unwrap();
}