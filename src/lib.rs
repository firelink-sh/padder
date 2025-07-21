//!
//! Highly efficient data formatting and padding crate for Rust.
//!
//! ...
//!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Alignment {
    Left,
    #[default]
    Right,
    Center,
}

impl Alignment {
    pub fn pad_range(&self, n: usize) -> (usize, usize) {
        match self {
            Self::Left => (0, n),
            Self::Right => (n, 0),
            // Whenever `n` is an odd number - this alignment will have one more
            // on the right side, n = 9 => (4, 5).
            Self::Center => (n / 2, n - n / 2),
        }
    }
}

pub trait Source {
    type Symbol;
    type Slice<'a>: ?Sized
    where
        Self: 'a;
    type Buffer;
    type Output;

    fn slice_to_fit<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a>;

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output;

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Buffer,
    );
}

impl Source for &str {
    type Symbol = char;
    type Slice<'a> = Self;
    type Buffer = String;
    type Output = String;

    // Since we are dealing with utf-8 characters and not directly bytes we need to
    // find the byte offsets for the chars we want to slice away.
    fn slice_to_fit<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        let mut st_byte: usize = 0;
        let mut ed_byte: usize = self.len();

        match mode {
            Alignment::Left => {
                ed_byte = self
                    .char_indices()
                    .nth(width)
                    .map(|(byte_offset, _)| byte_offset) // `_` is the char
                    .expect("the &str did not contain enough characters!");
            }
            Alignment::Right => {
                st_byte = self
                    .char_indices()
                    .rev()
                    // Since we are reversing for the start byte we want the
                    // byte offset of the char before - because it is inclusive.
                    .nth(width - 1)
                    .map(|(byte_offset, _)| byte_offset)
                    .expect("th &str did not contain enough characters!");
            }
            Alignment::Center => {
                // When slicing in this mode if the source is an even number
                // and the target width is an odd number - then an extra character
                // will be removed from the right of the &str.
                let n_chars_current: usize = self.chars().count();
                let n_l_skip: usize = n_chars_current / 2 - width / 2 - width % 2;
                let n_r_skip: usize = n_chars_current - n_chars_current / 2 + width / 2;
                for (idx, (byte_offset, _)) in self.char_indices().enumerate() {
                    if idx == n_l_skip {
                        st_byte = byte_offset;
                    }
                    if idx == n_r_skip {
                        ed_byte = byte_offset
                    }
                }
            }
        };

        &self[st_byte..ed_byte]
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        let n_chars_current: usize = self.chars().count();

        if width < n_chars_current {
            return self.slice_to_fit(width, mode).to_string();
        }

        let n_chars_diff = width - n_chars_current;
        if n_chars_diff == 0 {
            return self.to_string();
        }

        let n_bytes_target: usize = width * symbol.len_utf8();
        let mut output = String::with_capacity(n_bytes_target);
        let (n_l_pads, n_r_pads): (usize, usize) = mode.pad_range(n_chars_diff);

        (0..n_l_pads).for_each(|_| output.push(symbol));
        output.push_str(self);
        (0..n_r_pads).for_each(|_| output.push(symbol));

        output
    }

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Buffer,
    ) {
        let n_chars_current: usize = self.chars().count();

        if width < n_chars_current {
            buffer.push_str(self.slice_to_fit(width, mode));
            return;
        }

        let n_chars_diff = width - n_chars_current;
        if n_chars_diff == 0 {
            buffer.push_str(self);
        }

        let (n_l_pads, n_r_pads): (usize, usize) = mode.pad_range(n_chars_diff);
        (0..n_l_pads).for_each(|_| buffer.push(symbol));
        buffer.push_str(self);
        (0..n_r_pads).for_each(|_| buffer.push(symbol));
    }
}

/*
impl Source for String {
    type Symbol = char;
    type Slice<'a> = &'a str;
    type Buffer = String;
    type Output = String;

    fn slice_to_fit<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        match mode {
            Alignment::Left => &self[0..width],
            Alignment::Right => &self[(self.len() - width)..],
            Alignment::Center => {
                let st: usize = self.len() / 2 - width / 2;
                let ed: usize = self.len() / 2 + width / 2 + width % 2;
                &self[st..ed]
            }
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice_to_fit(width, mode).to_string();
        }

        let diff: usize = width - self.len();
        if diff == 0 {
            return self.clone();
        }

        let pad_range: PadRange = mode.pad_range(diff);
        let mut output = String::with_capacity(width);

        (0..pad_range.left()).for_each(|_| output.push(symbol));
        output.push_str(self);
        (0..pad_range.right()).for_each(|_| output.push(symbol));

        output
    }

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Buffer,
    ) {
        if width < self.len() {
            buffer.push_str(self.slice_to_fit(width, mode));
            return;
        }

        let diff: usize = width - self.len();
        if diff == 0 {
            buffer.push_str(self);
            return;
        }

        todo!()
        /*
        let pad_range: PadRange = mode.pad_range(diff);

        (0..pad_range.left()).for_each(|_| buffer.push(symbol));
        buffer.push_str(self);
        (0..pad_range.right()).for_each(|_| buffer.push(symbol));
        */
    }
}
*/

pub fn pad<S: Source>(source: S, width: usize, mode: Alignment, symbol: S::Symbol) -> S::Output {
    source.pad(width, mode, symbol)
}

pub fn pad_and_push_to_buffer<S: Source>(
    source: S,
    width: usize,
    mode: Alignment,
    symbol: S::Symbol,
    buffer: &mut S::Buffer,
) {
    source.pad_and_push_to_buffer(width, mode, symbol, buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_str_left() {
        let width: usize = 10;
        let source: &str = "Artorias";
        let output: String = pad(source, width, Alignment::Left, ' ');
        let expected: &str = "Artorias  ";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_right() {
        let width = 2;
        let source: &str = "radagon";
        let output: String = pad(source, width, Alignment::Right, '´');
        let expected: &str = "on";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_str_center() {
        let width = 6;
        let source: &str = "radahn";
        let output: String = pad(source, width, Alignment::Center, '$');
        let expected: &str = "radahn";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_and_push_to_buffer_str_left() {
        let width = 23;
        let source: &str = "liurna of the lakes";
        let mut buffer = String::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Left, '@', &mut buffer);
        let expected: &str = "liurna of the lakes@@@@";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_and_push_to_buffer_str_right() {
        let width = 1;
        let source: &str = "limgrave";
        let mut buffer = String::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Right, '*', &mut buffer);
        let expected: &str = "e";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_and_push_to_buffer_str_center() {
        let width = 16;
        let source: &str = "altus plateu";
        let mut buffer = String::with_capacity(width);
        pad_and_push_to_buffer(source, width, Alignment::Center, '¡', &mut buffer);
        let expected: &str = "¡¡altus plateu¡¡";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn str_pad_left() {
        let width: usize = 10;
        let source: &str = "Artorias";
        let output: String = source.pad(width, Alignment::Left, '🤠');
        let expected: &str = "Artorias🤠🤠";
        assert_eq!(expected, output);
    }

    #[test]
    fn str_pad_right() {
        let width: usize = 10;
        let source: &str = "kebab";
        let output: String = source.pad(width, Alignment::Right, '2');
        let expected: &str = "22222kebab";
        assert_eq!(expected, output);
    }

    #[test]
    fn str_pad_center() {
        let width: usize = 15;
        let source: &str = "astro bot!";
        let output: String = source.pad(width, Alignment::Center, '-');
        let expected: &str = "--astro bot!---";
        assert_eq!(expected, output);
    }

    #[test]
    fn str_pad_same_width() {
        let width: usize = 2;
        let source: &str = "¶¶";
        let output: String = source.pad(width, Alignment::Left, '¨');
        assert_eq!(source, output);
    }

    #[test]
    fn str_pad_sliced_center_odd() {
        let width: usize = 3;
        let source: &str = "  ¡@£   ";
        let output: String = source.pad(width, Alignment::Center, '¨');
        let expected: &str = "¡@£";
        assert_eq!(expected, output);
    }

    #[test]
    fn str_pad_sliced_center_even() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "§øł0k0";
        assert_eq!(expected, source.pad(width, Alignment::Center, 'x'));
    }

    #[test]
    fn str_pad_sliced_left() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "1¡§øł0";
        assert_eq!(expected, source.pad(width, Alignment::Left, 'x'));
    }

    #[test]
    fn str_pad_sliced_right() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "ł0k0äツ";
        assert_eq!(expected, source.pad(width, Alignment::Right, 'x'));
    }

    #[test]
    fn str_pad_to_buffer_left() {
        let width: usize = 8;
        let source: &str = "Solaire";
        let mut buffer: String = String::with_capacity(width);
        source.pad_and_push_to_buffer(width, Alignment::Left, '\\', &mut buffer);
        let expected: &str = "Solaire\\";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn str_pad_to_buffer_right() {
        let width: usize = 13;
        let source: &str = "gwyn";
        let mut buffer: String = String::with_capacity(width);
        source.pad_and_push_to_buffer(width, Alignment::Right, 'ö', &mut buffer);
        let expected: &str = "ööööööööögwyn";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn str_pad_to_buffer_center() {
        let width: usize = 15;
        let source: &str = "seKiro";
        let mut buffer: String = String::with_capacity(width);
        source.pad_and_push_to_buffer(width, Alignment::Center, 'ツ', &mut buffer);
        let expected: &str = "ツツツツseKiroツツツツツ";
        assert_eq!(expected, buffer);
    }

    /*
    #[test]
    fn string_pad_left() {
        let width: usize = 7;
        let source = String::from("coffee");
        let output: String = source.pad(width, Alignment::Left, ';');
        let expected: &str = "coffee;";
        assert_eq!(expected, output);
    }

    #[test]
    fn string_pad_right() {
        let width: usize = 9;
        let source = String::from("ps5");
        let output: String = source.pad(width, Alignment::Right, '±');
        let expected: &str = "±±±±±±ps5";
        assert_eq!(expected, output);
    }

    #[test]
    fn string_pad_center() {
        let width: usize = 31;
        let source = String::from("you are not prepared");
        let output: String = source.pad(width, Alignment::Center, '§');
        let expected: &str = "§§§§§you are not prepared§§§§§§";
        assert_eq!(expected, output);
    }

    #[test]
    fn string_pad_sliced() {
        let width: usize = 2;
        let source = String::from("123489700+8471983kbnlajvbroiaye87r687¨ä¨*ÄÂ*ÅWoU)P(FU893y");
        let output: String = source.pad(width, Alignment::Left, '|');
        let expected: &str = "12";
        assert_eq!(expected, output);
    }

    #[test]
    fn string_pad_same_width() {
        let width: usize = 3;
        let source = String::from("123");
        let output: String = source.pad(width, Alignment::Left, '8');
        assert_eq!(source, output);
    }
    */
}
