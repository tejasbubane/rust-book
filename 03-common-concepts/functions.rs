fn main() {
    another_function();
    add(3, 6);
    block_expression_example();
    println!("five = {}", five());
    println!("-12 + -3 = {}", add_return(-12, -3));
}

// snake_case is convention for function names
// defining functions before or after calling doesn't matter
fn another_function() {
    println!("Another function!");
}

// Must declare signatures for all parameters
fn add(x: u32, y: u32) {
    println!("x + y = {}", x + y);
}

// statements - perform operation do not return value
// let declarations are statements
// expressions - return value - 5 + 6
// function/macro call, block {...} is an expression
fn block_expression_example() {
    let y = {
        let x = 3;
        x + 1 // no semicolon for expressions
    };
    println!("y = {:?}", y);
}

// functions with return values
// return types declared after arrow
fn five() -> u32 { 5 }
fn add_return(x: i32, y: i32) -> i32 {
    x + y // return value needs to be expression - so no semicolon
}
// adding semicolon will make is statement and
// no return value - expressed by (), empty tuple
