# lib-math-utils

A small mathematical utilities library written in Rust. It provides common functions such as calculating the GCD (Greatest Common Divisor) and LCM (Least Common Multiple).

## Features

- `mcd(a, b)`: Calculates the greatest common divisor.
- `mcm(a, b)`: Calculates the least common multiple.

## Usage

Add to your `Cargo.toml`:

```toml
lib-math-utils = { path = "../lib-math-utils" }
```

```rust
use lib_math_utils::{mcd, mcm};

fn main() {
    println!("GCD of 12 and 18: {}", mcd(12, 18));
    println!("LCM of 12 and 18: {}", mcm(12, 18));
}
```
