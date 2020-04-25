fn main() {
    string_object();
    assignments();
    arguments();
    return_values();
}

fn string_object() {
    // string literal - immutable - stack allocated
    let _s = "hello";

    // String object - heap allocated
    // able to store unknown amount of text
    // can be mutated
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}
// String s is not in scope here

fn assignments() {
    let x = 5;
    let y = x;
    // since 5 is basic (size known), it is copied over in stack itself
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // move - reference copy but also invalidate previous variable
    // println!("{}, world!", s1); - s1 is invalid - this fails to compile
    // because rust invalidates first variable - it is called a `move`

    let s3 = s2.clone(); // deep copy - heap contents actually get copied - slow
    println!("s2 = {}, s3 = {}", s2, s3);
}

fn arguments() {
    let s = String::from("Foo");
    take_ownership(s);
    // cannot use s anymore since it is moved
} // s is moved so not dropped here

fn take_ownership(st: String) {
    println!("st = {}", st);
} // st is owned by this function and is dropped here since it goes out of scope

fn return_values() {
    let s1 = String::from("bar");
    let s2 = take_and_give_back(s1); // s1 moved to function & back in s2
    // so s1 remains invalid here
    println!("s2 = {}", s2);
}

fn take_and_give_back(st: String) -> String { st }
