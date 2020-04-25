# Ownership

### Rules - checked by compiler

* Each value in Rust has variable that's called its owner.
* There can be only one owner at a time.
* When owner goes out of scope, the value will be dropped.

### Stack

* last-in first-out
* fixed size - function arguments, pointers
* all basic literals stored here - exact data & size known at compile-time - hardcoded into binary

### Heap

* Complex data types - whose length not known at compile time
* Program needs to ask OS to allocate memory - OS returns pointer
* Program hands it back when it is done using the memory
* Historically this handing back of memory is done by GC keeping track of
  which variables are stale/unreachable
* Rust takes different approach & has no runtime GC

### Rust's approach to GC

* Memory is automatically allocated once variable is introduced in scope
* Rust calls `drop` when variable goes out of scope - library authors return memory in this function

```rust
let s1 = String::from("hello");
let s2 = s1;
```

* Problem - s1 & s2 might go out of scope at different times, and might end up freeing memory twice
  which is a bug.
* Instead of copying pointer, rust invalidates previous variable (s1)
* using s1 after `moving` it to s2 will throw compile error.

* basic literals (integers, string literals), etc (values of known memory length at compile time)
  are stored in stack itself. So the `move` doesn't apply to them.

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```
This will `NOT` give compile error - because x was `NOT` moved.

* no difference between deep & shallow copy for basic literals like integers (x & y above)
* passing a value to function also moves values - exactly like assignment.
* returning values also transfer ownership.

### References

* We can pass variables without passing ownership - using references.

```rust
foo(&s1)

fn foo(s: &String) {}
```

* references are immutable by default - you cannot modify what you don't own.
* `&mut s` will create mutable reference - but only one mutable reference allowed for a value
  this prevents data-races.

```rust
let r1 = &mut s;
let r2 = &mut s; // compile error
```

* Also mutable ref cannot exist along with another immutable one - code accessing immutable ref
  won't want sudden mutation from somewhere else.
* Multiple immutable refs can co-exist.
* New scopes can be created using curly braces `{..}` but still no two variables/references will co-exist.
