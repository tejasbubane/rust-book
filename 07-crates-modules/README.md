# Modules

```rust
mod foo {
    fn run_foo() {}
}

```

* Modules can be nested - modules can have multiple definitions functions, structs, enums, etc.
* All items in module are private by default.
* parents cannot use private items inside child modules, but child modules can use private items in
  parent module.
* Use `pub` keyword to expose items outside module.
* Marking module `pub` does not make all inner items public, we have to mark each one of them public.

#### Struct

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
```

* Making `struct` public does not make its fields public - we have to mark each of them.
* Cannot get or set `seasonal_fruit` in above example. We have to export constructor

```
impl Breakfast {
    pub fn sum(toast: & str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches")
        }
    }
}

let meal = Breakfast::summer("rye");
meal.toast // works
// meal.seasonal_fruit -- does not work
// same for setters (if meal is mutable)
```

#### Enum

* When `enum` is made public, all its variants become public.

## use

* Bring module in scope with `use module::path`
* Prefer not to bring functions directly into scope like
  `use module::path::function` instead do `use module::path` and use function like
  `path::function` - this makes it clear the function is not defined in here but imported,
  also avoids function name clashes.
* Imported modules are private - to allow callers to use the imported module as though it
  is defined in here - `pub use`

```rust
pub use crate::front_of_house::hosting;
```

* Bring multiple modules into scope in single line:

```rust
use std::{cmp::Order, io};
```

* Bring everything in scope using wildcard `*`:

```rust
use std::collections::*;
```

## external packages

In `Cargo.toml`

```toml
[dependencies]
rand = "0.5.5"
```

in code:

```rust
use rand::Rng;
```
