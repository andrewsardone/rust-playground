http://www.rustforrubyists.com/book/chapter-04.html

An example of using Rust's built-in test annotation — seemingly no arguments,
returns, or naming convention needed.

Compile with the --test flag (test harness with special main function):

    rustc --test testing.rs

```rust
#[test]
fn this_tests_code() {
    fail!("Fail!");
}
```

There's also a traditional [`assert` macro][stdma] that calls `fail` if the
given condition evaluates to false.

```rust
#[test]
fn this_tests_equality() {
    assert!(1 == 1);
}
```

[stdma]: http://static.rust-lang.org/doc/master/std/macros/macro.assert.html
