fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("Area of rectangle is {}", area1(width1, height1));

    let r = (30, 50);
    println!("Area of rectangle is {}", area2(r));

    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle is {:#?}", rect);
    println!("Area of rectangle is {}", area(&rect));

    // method on Rectangle struct
    println!("Area of rectangle is {}", rect.area());

    let rect2 = Rectangle { width: 40, height: 20 };
    println!("Can rect hold rect2? - {}", rect.can_hold(&rect2));

    let rect3 = Rectangle { width: 10, height: 20 };
    println!("Can rect hold rect3? - {}", rect.can_hold(&rect3));

    // syntax to call associated functions is ::
    println!("Square is {:?}", Rectangle::square(5));
}

fn area1(width: u32, height: u32) -> u32 { width * height }

fn area2(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }

#[derive(Debug)] // allows printing Rectangle instance using {:?} or {:#?}
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// area as method on Rectangle struct
impl Rectangle {
    // no need to give type of &self
    // self can also be borrowed mutably - (&mut self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // square is associated function not method
    // because it does not accept self as argument
    // these are usually constructor-style functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
// these methods/associated-functions can be split into multiple `impl` blocks
