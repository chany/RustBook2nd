#![allow(unused)]

fn main() {
    let len = String::from("Hola").len();
    println!("Hola 길이: {}", len);
    let hola = "Hola";
    //println!("{}", &hola[0]);

    let len = String::from("가나다라").len();
    println!("가나다라 길이: {}", len);
    let abc = "가나다라";
    //println!("{}", &abc[0]);

    //let s = &abc[0..1];
    //println!("{}", s);
    //let s = &abc[0..2];
    //println!("{}", s);
    let s = &abc[0..3];
    println!("{}", s);
    //let s = &abc[0..4];
    //println!("{}", s);

    for c in "가나다라".chars() {       // chars()
        println!("{}", c);
    }
    
    for b in "가나다라".bytes() {       // bytes()
        println!("{}", b);
    }
}