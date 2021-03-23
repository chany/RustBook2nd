use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();       // 새로운 송신자 생성. 같은 수신자에게 보내도록 clone()
    thread::spawn(move || {             // 스레드 1
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();     // tx1가 rx에게
            thread::sleep(Duration::new(1, 0));
        }
    });

    thread::spawn(move || {             // 스레드 2
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();      // tx가 rx에게
            thread::sleep(Duration::new(1, 0));
        }
    });

    for received in rx {        // 채널이 닫히면 자동으로 끝남
        println!("Got: {}", received);
    }
}