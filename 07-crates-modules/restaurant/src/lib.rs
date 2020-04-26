mod front_of_house;

// mod front_of_house {
//     // all module items are private by default
//     pub mod hosting { // making module pub does not make functions inside public
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
// Moved to separate files

// bring modules in scope by `use` keyword
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // because of use we brought hosting module in scope
    hosting::add_to_waitlist()
}

use std::collections::HashMap;

fn main() {
    eat_at_restaurant();

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map = {:?}", map);
}

// rename modules by `as` keyword
use std::io::Result as IoResult;
