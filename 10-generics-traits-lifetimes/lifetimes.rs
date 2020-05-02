fn main() {
    reference();

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest string is {}", result);

    {
        let string3 = String::from("xyz");
        let result = longest(string1.as_str(), string3.as_str());
        println!("Longest string is {}", result); // result is not valid outside
    }
    // moving result print here will cause compiler error

    longest_with_an_announcement("foo", "bar", "News");
}

// fn dangling_references() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//         println!("r: {}", r);
//     }
// }
// scope(lifetime) of x is less than reference r - so rust throws compile-time error

fn reference() {
    let x = 5; // lifetime of x is more than r - so r is allowed to refer to x
    let r = &x;
    println!("r: {}", r);
}

// 'a is the lifetime that x & y overlap (from where they are defined & passed in)
// returned value will live have lifetime 'a which means shorter among x & y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
