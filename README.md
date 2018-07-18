# Elias-Fano, in Rust
[![Build Status](https://img.shields.io/badge/crate-elias--fano-brightgreen.svg)](https://crates.io/crates/elias-fano)
[![Rust Docs](https://img.shields.io/badge/docs.rs-elias--fano-orange.svg)](https://docs.rs/elias-fano)

Elias-Fano encoding in Rust.

The Elias-Fano encoding scheme is a quasi-succinct compression method for monotonic integers using gap compression on a Bitset. It allows for decompression of a bit at any position in `O(1)` time complexity.

Being quasi-succinct, it is therefore almost as good as the best theoretical possible compression as determined by the [Shannon-Hartley](https://en.wikipedia.org/wiki/Shannon%E2%80%93Hartley_theorem) theorem.

This implementation is based largely on one written in Go by [Antonio Mallia](https://www.antoniomallia.it/), which can be found at his repository [amallia/go-ef](https://github.com/amallia/go-ef).

## Todo:
- [x] Tests
- [x] Example usage
- [ ] Benchmarks, comparison with other implementations

## Installation
Add the following line to your Cargo.toml:
```diff
[dependencies]
+ elias-fano = "0.2.6"
```

## Example Usage
```rust
extern crate elias_fano;

use elias_fano::EliasFano;

fn main() {
    let sorted_array = [0, 3, 40, 1000];
    let size = sorted_array.len();

    let mut ef = EliasFano::new(sorted_array[size - 1], size as u64);

    ef.compress(&sorted_array);

    println!("{}", ef.value()); // 1

    match ef.next() {
        Ok(val) => println!("Retrieved value: {}", val), // 3
        Err(error) => println!("Err: {}", error),        // Out of bounds
    }

    let _ = ef.next();
    println!("{}", ef.value()); // 40

    ef.reset();
    println!("{}", ef.value()); // 0

    let _ = ef.visit(3);
    println!("{}", ef.value()); // 1000
}
```

## License
MIT licensed, see LICENSE for more details.
