fn main() {
    let _v: Vec<i32> = Vec::new(); // type annotation required because no values inside

    let mut v = vec![1, 2, 3]; // type annotation not required
    println!("v = {:?}", v);

    // adding elements
    v.push(5);
    v.push(7);
    println!("v = {:?}", v);

    // reading elements
    // 1. via index
    let third: &i32 = &v[2]; // panic if element not found
    println!("Third element is {}", third);
    // 2. via get method
    match v.get(3) { // safe way to get even if element not found - no panic
        Some(fourth) => println!("Fourth is {}", fourth),
        None => println!("No fourth found"),
    }

    // Iterating
    for i in &v {
        println!("{}", i);
    }
    let mut v1 = vec![10, 20, 30];
    for i in &mut v1 {
        *i += 3; // * is dereference operator
    }
    println!("modified v1 is {:?}", v1);
} // vector is dropped when it goes out of scope
