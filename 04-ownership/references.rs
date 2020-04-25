fn main() {
    reference_pass();
    mutable_ref();
}

fn reference_pass() {
    let s1 = String::from("hello");
    let len = calc_length(&s1); // pass reference - ownership still remains here
    println!("Length of {} is {}", s1, len);
}
fn calc_length(s: &String) -> usize { s.len() }
// since s is reference (not owned) - value it points to will not be dropped

fn mutable_ref() {
    let mut s = String::from("hello-123");
    let len = calc_len2(&mut s);
    println!("Length of {} is {}", s, len);
}
fn calc_len2(st: &mut String) -> usize { st.len() }
