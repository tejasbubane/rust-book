# Enum

* `struct` are the `AND` & `enum` are the `OR` of algebraic data-types

```rust
enum IpAddrKind {
    V4,
    V6,
}
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

* IP address can either be V4 `OR` V6 - not both.
* We can mix & match enums & structs like sum & product ADTs in Haskell.

```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```

* `enum` variants can also hold values

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
let loopback = IpAddr::V6(String::from("::1"));
```

### Match

```rust
match x {
    Some(i) => i + 1,
    None => 0
}
```

* All matches are exhaustive - compile error if any case left out.
* Use `_` for wildcard match.

### If-let

* similar to `match`:

```rust
if let Some(x) = foo {
    x + 1
} else {
    0 // this is equivalent to _ in match
}
```
