# Elias-Fano, in Rust
[![Build Status](https://img.shields.io/badge/crate-elias--fano-brightgreen.svg)](https://crates.io/crates/elias-fano)

Elias-Fano encoding in Rust.

The Elias-Fano encoding scheme is a quasi-succinct compression method for monotonic integers using gap compression on a Bitset. It allows for decompression of a bit at any position in `O(1)` time complexity.

Being quasi-succinct, it is therefore almost as good as the best theoretical possible compression as determined by the [Shannon-Hartley](https://en.wikipedia.org/wiki/Shannon%E2%80%93Hartley_theorem) theorem.

This implementation is based largely on one written in Go by [Antonio Mallia](https://www.antoniomallia.it/) which can be found at his repository [amallia/go-ef](https://github.com/amallia/go-ef).

## Todo:
- [x] Tests
- [ ] Example usage
- [ ] Benchmarks, comparison with other implementations

## License
MIT licensed, see LICENSE for more details.
