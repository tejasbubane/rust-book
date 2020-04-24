fn main() {
    if_expressions(3);
    if_expressions(7);
    multi_if(6);
    multi_if(7);
    let _foo = if_statement();
    println!("Loop break = {}", loop_break());
    while_loop();
    for_loop();
    range();
}

fn if_expressions(number: i32) {
    if number < 5 { // condition must be bool - no auto-conversion
        println!("Number is less than 5");
    } else { // else is optional
        println!("Number is greater than 5");
    }
}

fn multi_if(number: i32) {
    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("number not divisible by 4, 3 or 2");
    }
}

// if is statement - so can be used on right side of assignment
// or as return value of function also
fn if_statement() -> i32 {
    let number = if true { 5 } else { 6 };
    println!("number = {}", number);
    number
}
// this also means both arms of if must have same type - otherwise compile error

// loop - runs forever - until Control-C termination
// use break
fn loop_break() -> u32 {
    let mut counter = 0;
    let result = loop { // loop is expression
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!");
}

// looping through collection
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }
}

fn range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
