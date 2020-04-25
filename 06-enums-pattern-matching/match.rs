enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_one(x: Option<u32>) -> u32 {
    match x {
        Some(i) => i + 1,
        None => 0
    }
}

fn main() {
    println!("penny = {}", value_in_cents(Coin::Penny));
    println!("nickel = {}", value_in_cents(Coin::Nickel));
    println!("dime = {}", value_in_cents(Coin::Dime));
    println!("quater = {}", value_in_cents(Coin::Quarter));

    println!("Add one 2 = {}", add_one(Some(2)));
    println!("Add one none = {}", add_one(None));

    let foo = Some(3);
    // if-let
    if let Some(3) = foo {
        println!("three!!");
    } else {
        // this is equivalent to _ in match
    }
}
