# Collections

### Vectors

* Implemented using generics: `Vec<T>` - only stores values of single type.
* Puts all values next to each other in memory.
* Adding new elements might require vector to be moved to new space - if there's no room left
  in consecutive memory to add new element.
* Borrowing rules come into play here - having immutable ref of any element - and trying to
  push new element won't work.

### Strings

* Implemented as collection of bytes.
* `String` is UTF-8 encoded
* Compiler can coerce `&String` and `&str` types.
* `String` is wrapper over a `Vec<u8>` - cannot access string by index because since it is a sequance of bytes, some UTF-8 chars might take more than one byte.
* slicing is allowed like `&s[0..3]` but is unsafe because graphemes can be different length.
  Accessing `[0..5]` in string of 3 bytes each will cause runtime panic.

### HashMap

```rust
use std::collections::HashMap;
```

* `HashMap<K, V>` stores mapping of keys of type `K` to values of type `V`.
* Inserting variables into HashMap `move` those values there & variables are no longer accessible.
