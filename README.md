<div align="center">

# padder
##### Highly efficient data formatting and padding crate for Rust.

</div>


TODOS:

- to support unicode-aware padding, look into using 'unicode-width'
  - or just go with `char.len_utf8()` and based on that calculate how much to allocate etc.
- to support numeric formatting, look into using 'num-format'
