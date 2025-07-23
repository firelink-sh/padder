<div align="center">

# padder
##### Highly efficient runtime data formatting and padding crate for Rust.

[![CI](https://github.com/firelink-sh/padder/actions/workflows/ci.yml/badge.svg)](https://github.com/firelink-sh/padder/actions/workflows/ci.yml)

</div>

padder is a lightweight Rust crate for padding and formatting data structures at runtime efficiently. It provides fine-grained control over alignment, truncating strategies, padding, and memory allocation - making it ideal for performance-critical applications.

Unlike the builtin `format!` macro, padder avoids unnecessary repeated heap allocations and lets you
pad and format directly into preallocated buffers.


## Features

- Pad strings, slices, and vectors with custom alignment and width.
- Zero-cost abstractions via the `Source` and `MutableSource` traits.
- Pad directly into buffers for fine-grained heap allocation control.
- Highly extensible to custom types through the provided traits.


## Installation

Add to you project with Cargo:

```
cargo add padder

(available features)
 - enable_unsafe
```


## Benchmarks

Run `cargo bench` to compare performance against the builtin `format!` macro. The benchmarks are
organized by feature and alignment strategy.

To benchmark the `enable_unsafe` feature to see if the use of `unsafe` optimizations actually
improves performance on your system, run `cargo bench --bench enable_unsafe --features
enable_unsafe`.


## License

padder is distributed under the terms of both the MIT license and the Apache License (version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

