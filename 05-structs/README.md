# Structs

* Collection of data - with named `fields`

```rust
struct User {
    email: String,
    age: u32,
}
```

* Instance of struct can be mutable as a whole, cannot mark only certain field mutable.

### Methods

* Similar to functions - arguments - return values
* Difference is they are defined within context of a struct
* first parameter is always `self` - which is the instance the method is being called on


```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

* functions inside `impl` need not always accept `self` - those that don't are called
  `associated functions` - these are like class functions.

```rust
Rectangle::square(5) // returns rectangle instance with height & width same = 5
```
