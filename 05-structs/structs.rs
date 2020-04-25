struct User { // struct has many `fields`
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs - used to name tuples - create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("tejas@ex.com"),
        username: String::from("tejas"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 email = {}", user1.email);

    let user2 = build_user(String::from("foo@bar.com"), String::from("foobar"));
    println!("user2 email = {}", user2.email);

    let user3 = copy_user(user2);
    println!("user3 email = {}", user3.email);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username, // shorthand for variables having same name as fields
        active: true,
        sign_in_count: 1,
    }
}

fn copy_user(user: User) -> User {
    User {
        active: true,
        sign_in_count: 1,
        ..user
    }
}
