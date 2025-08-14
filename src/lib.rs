//!
//! A highly efficient Rust crate for padding data during runtime.
//!
//! padder is a lightweight Rust crate for padding and formatting data structures at runtime
//! efficiently. It provides fine-grained control over alignment, truncating strategies, padding,
//! and memory allocation - making it ideal for performance-critical applications.
//!
//! Unlike the builtin `format!` macro, padder avoids unneccessary repeated heap allocations and
//! lets you pad and format directly into preallocated buffers, or modify buffers in-place.
//!
//! **Fully UTF-8 compatible** - padder works seamlessly with any Unicode characters like emojis (🐉),
//! Japanese kana/kanji (こんにちは), or other multibyte symbols, when operating on String types.
//!
//! # Features
//! - Pad strings, slices, and vectors with custom alignment and width.
//! - Zero-cost abstractions via the `Source` and `MutableSource` traits.
//! - Pad directly into buffers for fine-grained heap allocation control.
//! - Highly extensible to custom types through the provided traits.
//!
//! # Usage
//! ```
//! use padder::*;
//!
//! let mut string = String::from("kratos");
//! (&mut string).pad(10, Alignment::Right, '$');  // "$$$$kratos"
//! pad_mut(&mut string, 14, Alignment::Center, '-');  // "--$$$$kratos--"
//!
//! let s = "dragon";
//! let padded = pad(s, 11, Alignment::Left, '🐉');
//! assert_eq!("dragon🐉🐉🐉🐉🐉", padded);
//!
//! let vec: Vec<u8> = Vec::from(&[0u8, 2, 5]);
//! let mut buffer: Vec<u8> = Vec::with_capacity(5);
//! pad_to_buffer(vec, 5, Alignment::Right, 128u8, &mut buffer);
//! assert_eq!(Vec::from(&[128u8, 128, 0, 2, 5]), buffer);
//! ```
//!

mod alignment;
mod mutable_source;
mod source;

pub use alignment::{Alignment, Pads};
pub use mutable_source::MutableSource;
pub use source::Source;

/// Pads the given source buffer to the specified `width` using the provided `symbol` and alignment `mode`.
///
/// This is a convenience wrapper around the [`Source::pad`] method. It consumes the source buffer
/// and produces a new, owned, and padded buffer.
///
/// # Examples
/// ```
/// use padder::*;
///
/// let string = String::from("this is a string");
/// let padded = pad(string, 20, Alignment::Right, '_');
/// assert_eq!("____this is a string", padded);
///
/// let string = String::from("sackboy");
/// let padded = pad(string, 4, Alignment::Left, ' ');
/// assert_eq!("sack", padded);  // 4 < string.chars.count() so it will be truncated
///
/// let vec: Vec<usize> = Vec::from(&[200, 10, 23]);
/// let padded = pad(vec, 6, Alignment::Center, 0usize);
/// assert_eq!(Vec::from(&[0usize, 200, 10, 23, 0, 0]), padded);
/// ```
pub fn pad<S: Source>(source: S, width: usize, mode: Alignment, symbol: S::Symbol) -> S::Output {
    source.pad(width, mode, symbol)
}

/// Pad the given mutable source buffer in-place to the specified `width` using the provided `symbol` and `alignment` mode.
///
/// This is a convenience wrapper around the [`MutableSource::pad`] method. It modifies the source
/// buffer directly and does not consume it.
///
/// # Examples
/// ```
/// use padder::*;
///
/// let mut string = String::from("elden ring");
/// pad_mut(&mut string, 14, Alignment::Left, '💍');
/// assert_eq!("elden ring💍💍💍💍", string);
///
/// let mut string = String::from("dark souls");
/// pad_mut(&mut string, 14, Alignment::Center, '🌑');
/// assert_eq!("🌑🌑dark souls🌑🌑", string);
/// ```
pub fn pad_mut<S: MutableSource>(mut source: S, width: usize, mode: Alignment, symbol: S::Symbol) {
    source.pad(width, mode, symbol);
}

/// Pad the given source to match the specified `width` according to the specified alignment
/// `mode` by writing into the provided `buffer`.
///
/// This is a convenience wrapper around the [`Source::pad_to_buffer`] method. It does not modify
/// the source buffer and instead directly writes in the the `buffer`.
///
/// # Examples
/// ```
/// use padder::*;
///
/// let slice: &[&str] = &[
///     "liurnia",
///     "of",
///     "the",
///     "lakes",
/// ];
/// let width = 6;
/// let mut buf: Vec<&str> = Vec::with_capacity(width);
/// pad_to_buffer(slice, width, Alignment::Right, "tarnished", &mut buf);
/// let expected: Vec<&str> = Vec::from(&[
///     "tarnished",
///     "tarnished",
///     "liurnia",
///     "of",
///     "the",
///     "lakes",
/// ]);
/// assert_eq!(expected, buf);
/// ```
pub fn pad_to_buffer<S: Source>(
    source: S,
    width: usize,
    mode: Alignment,
    symbol: S::Symbol,
    buffer: &mut S::Buffer,
) {
    source.pad_to_buffer(width, mode, symbol, buffer);
}

#[cfg(test)]
mod tests_wrappers {
    use super::*;

    #[test]
    fn str_pad() {
        let output = pad("little big planet", 20, Alignment::Center, '!');
        assert_eq!("!little big planet!!", output);
        assert_eq!(20, output.len());
    }

    #[test]
    fn string_pad() {
        let output = pad("uh oh", 8, Alignment::Right, '💣');
        assert_eq!("💣💣💣uh oh", output);
        assert_eq!(17, output.len());
    }

    #[test]
    fn mut_string_pad() {
        let mut s = String::from("def fn(xd: str) -> None: ...");
        pad_mut(&mut s, 30, Alignment::Left, 'æ');
        assert_eq!("def fn(xd: str) -> None: ...ææ", s);
        assert_eq!(32, s.len());
        assert_eq!(30, s.chars().count());
    }

    #[test]
    fn vec_pad_to_buffer() {
        let mut buffer: Vec<u8> = Vec::new();
        let v: Vec<u8> = Vec::from(&[1u8, 2]);
        assert_eq!(2, v.len());
        pad_to_buffer(v, 4, Alignment::Right, 89u8, &mut buffer);
        assert_eq!(Vec::from(&[89u8, 89, 1, 2]), buffer);
        assert_eq!(4, buffer.len());
    }

    #[test]
    fn slice_pad() {
        let source: &[bool] = &[true, false, true, true];
        let s = source.pad(6, Alignment::Center, false);
        assert_eq!(Vec::from(&[false, true, false, true, true, false]), s);
        assert_eq!(6, s.len());
    }
}
