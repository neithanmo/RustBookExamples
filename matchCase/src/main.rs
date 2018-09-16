#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("quarter contains {:?}", state);
            25
        },
        _ => 504,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");

    println!("result {}", value_in_cents(Coin::Quarter( UsState::Alaska)) );
    println!("result {}", value_in_cents(Coin::Dime) );

    let five = Some(5);
    println!("que es: {:?}", five);
    let six = plus_one(five);
    println!("que es: {:?}", six);
    let none = plus_one(None);
    println!("que es: {:?}", none);
}
