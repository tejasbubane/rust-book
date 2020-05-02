# Generics

* Types have placeholders which allows us to write generic code that works with multiple types.

```rust
fn foo<T>(&[T]) -> T {}
// function takes reference to array of T elements & return one value of type T

struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* Runtime cost of generics is none because rust uses `monomorphization`.
* This is a technique compiler will see all occurrences and generate duplicate code for every type.
  eg. `enum Option_i32` and `enum Option_f64`
* So code runs just as if programmer had duplicated it by hand.

# Traits

* Tells compiler about functionality a particular type can share with other types.

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

* Abstract methods - need to be implemented while implementing traits on specific types.
* Can also define implementation inside trait - this will serve as defailt implementation
  which can be kept or overriden by implementing types.
* If importing traits, they need to be public to implement.
* We can implement trait only if either the trait or type is local to our crate -
  cannot implement third-party trait on third-party types.
* Trait methods can call other methods in trait - even those without default implementation.

* Function arguments can be trait bound:

```rust
fn notify(item1: impl Summary, item2: impl Summary) {...};

// or

fn nofity<T: Summary>(item1: T, item2: T) {...};
```

* Use `+` syntax for multiple bounds:

```rust
fn notify(item: impl Summary + Display) {...};
```

* For lot of clauses use `where` syntax - just to make function signature more clear:

```rust
fn some_function<T,U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{...}
```

* Can also add trait on return type:

```rust
fn return_summarizable() -> impl Summary { ... };
```

# Lifetime

* Scope is the lifetime of variable - rust does not allow borrowing out-of-scope references
* Function returning references have to specify lifetime of the reference.

```rust
&i32
&'a i32
&'a mut i32
```

* These annotations tell rust how generic lifetime parameters of multiple references relate.
  Two parameters with `'a` tell that they both have same lifetime.

* Structs can hold references (as opposed to owned types) - we need lifetimes

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```
means an instance of `ImportantExcerpt` cannot outlive the reference it holsd in its `part` field.

* lifetime parameters are part of `type`

* `'static` lifetime means this reference can live for entire duration of the program.
* All string literals have `'static` lifetime - because text of those string literals is stored
  directly in program binary.
