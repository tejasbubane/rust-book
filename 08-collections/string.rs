fn main() {
    let _st: &str = "hello";
    let mut s = String::new();

    let data = "initial contents";
    let s1 = data.to_string(); // available on any type that implements Display trait
    let s2 = "initial contents".to_string(); // same as above
    let s3 = String::from("initial contents");

    s.push_str("bar"); // append string slice
    println!("s = {}", s);

    s.push('l');
    println!("s = {}", s); // appends single char

    let s4 = s1 + " " + &s2 + " " + &s3; // first one has to be String not &str
    // since s1 is moved, we cannot use it anymore
    // s2 and s3 are not moved, they are still valid
    println!("s4 = {}", s4);

    let s5 = format!("{}-{}", s2, s3);
    // no ownership transfer with format
    println!("s2 = {}, s3 = {}, s5 = {}", s2, s3, s5);

    // Indexing
    // let h = s2[0]; -- this fails - because string is set of bytes
    // & UTF-8 might vary byte-length of chars
    let hello = "नमस्ते";
    let s = &hello[0..3]; // each char in hello is 3 bytes - so this works
    // but &hello[0..4] would cause runtime panic
    println!("s = {}", s);
    // slicing is unsafe

    for c in hello.chars() { // this is safe
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}
