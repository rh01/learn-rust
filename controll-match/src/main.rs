#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1 // 返回值不必加分号
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn main() {
    let st1 = UsState::Alabama;
    let c1 = Coin::Quarter(st1);
    println!("coin is {}", value_in_cents(c1));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // 然而，match 在只关心 一个 情况的场景中可能就有点啰嗦了。为此 Rust 提供了if let。
}
