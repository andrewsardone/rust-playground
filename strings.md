## String formatting

The `format!` function gives you a printf-like formatted output conversion.

```rust
use std::io::println;

fn main() {
    for num in range(1, 4) {
        // type-checked at compile time!
        println(format!("{:d}", num));
    }
}
```

`println!` == a macro of `println` and `format!`

```rust
fn main() {
    for num in range(1, 4) {
        println!("{:d}", num);
    }
}
```

No need to import the `io` module.
