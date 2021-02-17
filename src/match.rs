fn main() {
    let x = 3;

    match x {
        1 => println!("one"),
        2 => {
            println!("two");
            println!("two");
        }  // <- { }로 묶여 있을 때는 comma 없어도 됨
        _ => println!("no match"), // match되지 않는 나머지
    }

    //
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("no match");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),   // 값을 포함하도록
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny   => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("None");
            None
        },
        Some(i) => {
            println!("{}", i);
            Some(i + 1)
        },
    }
}
