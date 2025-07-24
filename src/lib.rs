//!
//! Highly efficient data formatting and padding crate for Rust.
//!
//! ...
//!

mod alignment;
mod mutable_source;
mod source;

pub use alignment::{Alignment, Pads};
pub use mutable_source::MutableSource;
pub use source::Source;

///
pub fn pad<S: Source>(source: S, width: usize, mode: Alignment, symbol: S::Symbol) -> S::Output {
    source.pad(width, mode, symbol)
}

///
pub fn pad_mut<S: MutableSource>(source: &mut S, width: usize, mode: Alignment, symbol: S::Symbol) {
    source.pad(width, mode, symbol);
}

///
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
        let output = "little big planet".pad(20, Alignment::Center, '!');
        assert_eq!("!little big planet!!", output);
        assert_eq!(20, output.capacity());
        assert_eq!(20, output.len());
    }

    #[test]
    fn string_pad() {
        let output = String::from("uh oh").pad(8, Alignment::Right, '💣');
        assert_eq!("💣💣💣uh oh", output);
        assert_eq!(17, output.capacity());
        assert_eq!(17, output.len());
    }

    #[test]
    fn mut_string_pad() {
        let mut s = String::from("def fn(xd: str) -> None: ...");
        (&mut s).pad(30, Alignment::Left, 'æ');
        assert_eq!("def fn(xd: str) -> None: ...ææ", s);
        assert_eq!(32, s.capacity());
        assert_eq!(32, s.len());
        assert_eq!(30, s.chars().count());
    }

    #[test]
    fn vec_pad_to_buffer() {
        let mut buffer: Vec<u8> = Vec::new();
        let v: Vec<u8> = Vec::from(&[1u8, 2]);
        v.pad_to_buffer(4, Alignment::Right, 89u8, &mut buffer);
        assert_eq!(Vec::from(&[89u8, 89, 1, 2]), buffer);
        assert_eq!(4, buffer.capacity());
        assert_eq!(4, buffer.len());
        assert_eq!(2, v.capacity());
        assert_eq!(2, v.len());
    }

    #[test]
    fn slice_pad() {
        let source: &[bool] = &[true, false, true, true];
        let s = source.pad(6, Alignment::Center, false);
        assert_eq!(Vec::from(&[false, true, false, true, true, false]), s);
        assert_eq!(6, s.capacity());
        assert_eq!(6, s.len());
    }
}
