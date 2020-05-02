fn main() {
    let max1 = largest_i32(&[12, 19, 9, 48, 1, 65, 15]);
    println!("max1 = {}", max1);
    let max2 = largest_i32(&[19, 10, 42, 5]);
    println!("max2 = {}", max2);

    let max_char = largest_char(&['b', 'k', 'a']);
    println!("max_char = {}", max_char);

    let p1 = Point { x: 5, y: 10.2 };
    let p2 = Point { x: "Hello", y: 'c' };
    println!("p.x = {}", p1.x());
    println!("mixup = {:?}", p1.mixup(p2));

    // fixing largest function to use trait
    let max3 = largest(&[12, 14, 19, 3, 10]);
    println!("max3 = {}", max3);
    let max4 = largest(&[12.3, 14.2, 19.1, 3.0, 10.9]);
    println!("max4 = {}", max4);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    };
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &letter in list {
        if letter > largest {
            largest = letter
        }
    };
    largest
}

// This is how you would define a function, but this won't compile just yet - we need traits
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     };
//     largest
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // Copy trait required because data copied into variable here
    for &item in list {
        if item > largest {
            largest = item;
        }
    };
    largest
}
