fn main() {
    // every value in rust has data type - either explicit or inferred
    let _guess: u32 = "42".parse().expect("Not a number!");
    // variables with underscore don't trigger unused variable warnings

    // Integers
    // signed(i8...i128, isize) & unsigned(u8...u128, usize)
    // isize & usize will be 32 & 64 bit based on OS architecture

    println!("\nFloating points");
    let x = 2.1; // f64 double precision by default
    let y: f32 = 3.2;
    println!("x = {}, y = {}", x, y);
    // let z = x + 1; -- float + integer not allowed

    println!("\nBoolean");
    let t = true;
    let f: bool = false;
    println!("t = {}, f = {}", t, f);

    println!("\nCharacters"); // 4 bytes in size, support unicode
    let c = 'z';
    let z = '😻'; // heart-eyed-cat emoji
    println!("c = {}, z = {}", c, z);

    println!("\nTuples"); // fixed length
    let tup: (i32, f64, char) = (10, 20.2, 'c');
    println!("tup = {:?}", tup); // {:?} is for pretty-print
    let (x, y, z) = tup; // destructure tuple
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("char is {}", tup.2); // access tuple value using dot index

    println!("\nArrays");
    let a = [1, 2, 3, 4]; // every element must have same type - fixed length
    // arrays are allocated on stack (not heap)
    let b: [i32; 2] = [10, 20]; // type contains length of array
    let rep = [3; 5]; // 3 five times - concise way of repeating elements
    println!("a = {:?}, b = {:?}, rep = {:?}", a, b, rep);
    // access array elements using [index]
    println!("Firsts of a = {}, b = {}, rep = {}", a[0], b[0], rep[0]);
    // a[10] -- invalid array element access will result in panic
}
