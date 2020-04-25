fn main() {
    let s = String::from("tejas bubane bangalore");
    println!("First word position is {}", first_word(&s));

    // with slices
    println!("First word is {}", first_word_slice(&s));

    // array slices
    let a = [1, 2, 3, 4, 5];
    let first_three: &[i32] = &a[..3];
    println!("First 3 are {:?}", first_three);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b is byte literal syntax
            return i
        }
    }
    s.len()
} // but this index is no way related to string which can mutate leaving result index invalid

// slice is reference to portion of string

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
