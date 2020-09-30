#![allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime(UsState),
    Quarter,
    Dollar,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado,
    California,
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime(UsState::Colorado)));
    println!("-----------------------------------");
    println!("{}", value_in_cents(Coin::Dollar));

    let five = Some(5);
    let _six = plus_one(five); // not ints but Option<i32> need to be converted for use with regular types
    let _none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(state) => {
            println!("Dime from {:?}", state);
            10
        }
        Coin::Quarter => 25,
        _ => {
            // _ represents default if no other tests pass
            println!("Unsupported Coin");
            0
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // needs to handle None or compiler will fail
        Some(i) => Some(i + 1),
    }
}
