<div align="center">

# padder
##### A highly efficient Rust crate for padding data during runtime.

[![MSRV](https://img.shields.io/badge/MSRV-1.85.1-orange)](https://crates.io/crates/padder)
[![CI](https://github.com/firelink-sh/padder/actions/workflows/ci.yml/badge.svg)](https://github.com/firelink-sh/padder/actions/workflows/ci.yml)
[![Tests](https://github.com/firelink-sh/padder/actions/workflows/tests.yml/badge.svg)](https://github.com/firelink-sh/padder/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/firelink-sh/padder/graph/badge.svg?token=OTFIM6UICZ)](https://codecov.io/gh/firelink-sh/padder)

<br>

</div>

padder is a lightweight Rust crate for padding and formatting data structures at runtime efficiently. It provides fine-grained control over alignment, truncating strategies, padding, and memory allocation - making it ideal for performance-critical applications.

Unlike the builtin `format!` macro, padder avoids unnecessary repeated heap allocations and lets you
pad and format directly into preallocated buffers.

**Fully UTF-8 compatible** - padder works seamlessly with any Unicode characters like emojis (🐉), Japanese kana/kanji (こんにちは), or other multibyte symbols when operating on String types.

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


## Usage

### Immutable padding

```rust
use padder::*;

// Padding an immutable string slice.
let s: &str = "radagon";
let padded: String = s.pad(11, Alignment::Left, '🐉');
assert_eq!("radagon🐉🐉🐉🐉", padded);

// Padding an immutable vector into a preallocted buffer.
let width: usize = 10;
let vec: Vec<u8> = Vec::from(&[0u8, 1, 2, 3, 4]);
let mut buf: Vec<u8> = Vec::with_capacity(width);
vec.pad_to_buf(width, Alignment::Right, 0u8, &mut buf);
assert_eq!(
    Vec::from(&[0u8, 0, 0, 0, 0, 0, 1, 2, 3, 4]),
     buf,
);
```


### In-place padding

```rust
use padder::*;

// Padding a mutable string in-place.
let mut ms = String::from("yharnam");
(&mut ms).pad(10, Alignment::Center, '👽');
assert_eq!("👽yharnam👽👽", ms);

// We can pad again! This time using the wrapper function `pad_mut`.
pad_mut(&mut ms, 13, Alignment::Right, '!');
assert_eq!("!!!👽yharnam👽👽", ms);

// And now we can truncate the string -
// (the symbol has no effect when truncating).
(&mut ms).pad(2, Alignment::Left, ' ');
assert_eq!("!!", ms);
```


## Examples

Take a look in [examples/](./examples) to see some short examples of how to use this crate.

To test out any of the examples you can run `cargo run -p <example-name>`.


## Benchmarks

Run `cargo bench` to compare performance against the builtin `format!` macro. The benchmarks are organized by source type, feature, and alignment strategy.

To benchmark the `enable_unsafe` feature to verify that the use of `unsafe` actually improves performance on your system, run the following: 

```
cargo bench --bench enable_unsafe --features enable_unsafe
```


## License

padder is distributed under the terms of both the MIT License and the Apache License (version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

