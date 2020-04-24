fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6; // this is allowed only because x is mut
    println!("The new value of x is {}", x);

    // constants
    // should always be type annotated
    // must be static value - result of function call or any other value
    // computed at runtime is not allowed.
    const MAX_POINTS: u32 = 100_000;
    // const FAILURE: u32 = result(); - fails because value is not static
    // const FAILURE = 123; - fails because type not provided
    println!("Max points allowed = {}", MAX_POINTS);

    // shadowing
    let foo = 5;
    println!("foo = {}", foo);
    let foo = "tejas"; // shadowing can change type also
    println!("foo = {}", foo);
    // changing type won't work with mut
}
