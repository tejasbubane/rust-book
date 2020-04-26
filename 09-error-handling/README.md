# Error Handling


* Rust does not have exceptions - two ways to handle errors:

1. `panic` - fail immediately - crash program with error
2. return `Result` type - which embeds, success/error data - handle this.

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* Unlike exceptions, `Result` is a type which compiler can check & callers must handle.
