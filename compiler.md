`rustc(1)` is the Rust language compiler.

## Semicolon and the value of an expression

Take the following:

```rust
use std::io::println;

fn main() {
    println(hello_world());
}

fn hello_world() -> &str {
    "Hello, world";
}
```

Compiling produces the following error:

```
❯ rustc foo.rs
foo.rs:7:1: 9:2 error: not all control paths return a value
foo.rs:7 fn hello_world() -> &str {
foo.rs:8     "Hello, world";
foo.rs:9 }
error: aborting due to previous error
```

Drop the semicolon and the expression ceases to be a *statement*, and hence we
can use its value (namely, as the return value since it's the last expression).

See the [FizzBuzz chapter][rfrfb] of *Rust for Rubyists* for a good
explanation.

[rfrfb]: http://www.rustforrubyists.com/book/chapter-05.html
