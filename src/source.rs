use crate::alignment::Alignment;

/// Immutable source to pad.
pub trait Source {
    type Symbol;
    type Buffer;
    type Output;
    type Slice<'a>: ?Sized
    where
        Self: 'a;

    fn slice<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a>;
    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output;
    fn pad_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Output,
    );
}

impl Source for &str {
    type Symbol = char;
    type Buffer = String;
    type Output = String;
    type Slice<'a> = Self;

    fn slice<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        let mut st_byte: usize = 0;
        let mut ed_byte: usize = self.len();

        match mode {
            Alignment::Left => {
                ed_byte = self
                    .char_indices()
                    .nth(width)
                    .map(|(byte_offset, _)| byte_offset)
                    .expect("the String did not contain enough chars!");
            }
            Alignment::Right => {
                st_byte = self
                    .char_indices()
                    .rev()
                    .nth(width - 1)
                    .map(|(byte_offset, _)| byte_offset)
                    .expect("the String did not contain enough chars!");
            }
            Alignment::Center => {
                let st_idx: usize = (self.chars().count() - width) / 2;
                let ed_idx: usize = st_idx + width;

                for (idx, (byte_offset, _)) in self.char_indices().enumerate() {
                    if idx == st_idx {
                        st_byte = byte_offset;
                    }
                    if idx == ed_idx {
                        ed_byte = byte_offset;
                    }
                }
            }
        };

        &self[st_byte..ed_byte]
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        let n_chars_current: usize = self.chars().count();
        if width < n_chars_current {
            return self.slice(width, mode).to_string();
        }

        let n_chars_diff: usize = width - n_chars_current;
        if n_chars_diff == 0 {
            return self.to_string();
        }

        let n_bytes_required: usize = self.len() + n_chars_diff * symbol.len_utf8();
        let mut output = String::with_capacity(n_bytes_required);
        let pads = mode.pads(n_chars_diff);

        (0..pads.left()).for_each(|_| output.push(symbol));
        output.push_str(self);
        (0..pads.right()).for_each(|_| output.push(symbol));

        output
    }

    fn pad_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Output,
    ) {
        let n_chars_current: usize = self.chars().count();
        if width < n_chars_current {
            buffer.push_str(self.slice(width, mode));
            return;
        }

        let n_chars_diff: usize = width - n_chars_current;
        if n_chars_diff == 0 {
            buffer.push_str(self);
            return;
        }

        // Preemptively allocate extra memory before padding.
        buffer.reserve_exact(n_chars_diff * symbol.len_utf8());

        let pads = mode.pads(n_chars_diff);
        (0..pads.left()).for_each(|_| buffer.push(symbol));
        buffer.push_str(self);
        (0..pads.right()).for_each(|_| buffer.push(symbol));
    }
}

impl Source for String {
    type Symbol = char;
    type Buffer = Self;
    type Output = Self;
    type Slice<'a> = &'a str;

    fn slice<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        let mut st_byte: usize = 0;
        let mut ed_byte: usize = self.len();

        match mode {
            Alignment::Left => {
                ed_byte = self
                    .char_indices()
                    .nth(width)
                    .map(|(byte_offset, _)| byte_offset)
                    .expect("the String did not contain enough chars!");
            }
            Alignment::Right => {
                st_byte = self
                    .char_indices()
                    .rev()
                    // Since we are reversing from the start byte we want the byte offset
                    // of the previous char - because the slice range is inclusive.
                    .nth(width - 1)
                    .map(|(byte_offset, _)| byte_offset)
                    .expect("the String did not contain enough chars!");
            }
            Alignment::Center => {
                // If `self.chars().count() - width` is an odd number - then the
                // extra character will be removed from the right.
                let st_idx: usize = (self.chars().count() - width) / 2;
                let ed_idx: usize = st_idx + width;

                for (idx, (byte_offset, _)) in self.char_indices().enumerate() {
                    if idx == st_idx {
                        st_byte = byte_offset;
                    }
                    if idx == ed_idx {
                        ed_byte = byte_offset;
                    }
                }
            }
        };

        &self[st_byte..ed_byte]
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        let n_chars_current: usize = self.chars().count();
        if width < n_chars_current {
            return self.slice(width, mode).to_string();
        }

        let n_chars_diff: usize = width - n_chars_current;
        if n_chars_diff == 0 {
            return self.clone();
        }

        let n_bytes_required: usize = self.len() + n_chars_diff * symbol.len_utf8();
        let mut output = String::with_capacity(n_bytes_required);
        let pads = mode.pads(n_chars_diff);

        (0..pads.left()).for_each(|_| output.push(symbol));
        output.push_str(self);
        (0..pads.right()).for_each(|_| output.push(symbol));

        output
    }

    fn pad_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Output,
    ) {
        let n_chars_current: usize = self.chars().count();
        if width < n_chars_current {
            buffer.push_str(self.slice(width, mode));
            return;
        }

        let n_chars_diff: usize = width - n_chars_current;
        if n_chars_diff == 0 {
            buffer.push_str(self);
            return;
        }

        // We don't have to check that the capacity of the `buffer` is too
        // small - because the `reserve_exact()` method checks this for us
        // and does nothing if it is already at enough capacity.
        //
        // This effectively becomes a no-op in such a case.
        buffer.reserve_exact(n_chars_diff * symbol.len_utf8());
        let pads = mode.pads(n_chars_diff);

        (0..pads.left()).for_each(|_| buffer.push(symbol));
        buffer.push_str(self);
        (0..pads.right()).for_each(|_| buffer.push(symbol));
    }
}

impl<T> Source for Vec<T>
where
    T: Copy + Sized,
{
    type Symbol = T;
    type Buffer = Vec<T>;
    type Output = Vec<T>;
    type Slice<'a>
        = &'a [T]
    where
        T: 'a;

    fn slice<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        match mode {
            Alignment::Left => &self[..width],
            Alignment::Right => &self[(self.len() - width)..],
            Alignment::Center => {
                let st_idx: usize = (self.len() - width) / 2;
                let ed_idx: usize = st_idx + width;
                &self[st_idx..ed_idx]
            }
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice(width, mode).to_vec();
        }

        let n_bytes_diff: usize = width - self.len();
        if n_bytes_diff == 0 {
            return self.clone();
        }

        let mut output = Vec::with_capacity(width);
        let pads = mode.pads(n_bytes_diff);

        (0..pads.left()).for_each(|_| output.push(symbol));
        output.extend_from_slice(self);
        (0..pads.right()).for_each(|_| output.push(symbol));

        output
    }

    fn pad_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Output,
    ) {
        if width < self.len() {
            buffer.extend_from_slice(self.slice(width, mode));
            return;
        }

        let n_bytes_diff: usize = width - self.len();
        if n_bytes_diff == 0 {
            buffer.extend_from_slice(self);
            return;
        }

        buffer.reserve_exact(n_bytes_diff);
        let pads = mode.pads(n_bytes_diff);

        (0..pads.left()).for_each(|_| buffer.push(symbol));
        buffer.extend_from_slice(self);
        (0..pads.right()).for_each(|_| buffer.push(symbol));
    }
}

impl<T> Source for &[T]
where
    T: Copy + Sized,
{
    type Symbol = T;
    type Buffer = Vec<T>;
    type Output = Vec<T>;
    type Slice<'a>
        = &'a [T]
    where
        T: 'a;

    fn slice<'a>(&'a self, width: usize, mode: Alignment) -> Self::Slice<'a> {
        match mode {
            Alignment::Left => &self[..width],
            Alignment::Right => &self[(self.len() - width)..],
            Alignment::Center => {
                let st_idx: usize = (self.len() - width) / 2;
                let ed_idx: usize = st_idx + width;
                &self[st_idx..ed_idx]
            }
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output {
        if width < self.len() {
            return self.slice(width, mode).to_vec();
        }

        let n_bytes_diff: usize = width - self.len();
        if n_bytes_diff == 0 {
            return self.to_vec();
        }

        let mut output = Vec::with_capacity(width);
        let pads = mode.pads(n_bytes_diff);

        (0..pads.left()).for_each(|_| output.push(symbol));
        output.extend_from_slice(self);
        (0..pads.right()).for_each(|_| output.push(symbol));

        output
    }

    fn pad_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Output,
    ) {
        if width < self.len() {
            buffer.extend_from_slice(self.slice(width, mode));
            return;
        }

        let n_bytes_diff: usize = width - self.len();
        if n_bytes_diff == 0 {
            buffer.extend_from_slice(self);
            return;
        }

        buffer.reserve_exact(n_bytes_diff);
        let pads = mode.pads(n_bytes_diff);

        (0..pads.left()).for_each(|_| buffer.push(symbol));
        buffer.extend_from_slice(self);
        (0..pads.right()).for_each(|_| buffer.push(symbol));
    }
}

#[cfg(test)]
mod tests_str {
    use super::*;

    #[test]
    fn pad_left() {
        let width: usize = 10;
        let source: &str = "Artorias";
        let output: String = source.pad(width, Alignment::Left, '🤠');
        let expected: &str = "Artorias🤠🤠";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_right() {
        let width: usize = 10;
        let source: &str = "kebab";
        let output: String = source.pad(width, Alignment::Right, '2');
        let expected: &str = "22222kebab";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center() {
        let width: usize = 15;
        let source: &str = "astro bot!";
        let output: String = source.pad(width, Alignment::Center, '-');
        let expected: &str = "--astro bot!---";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_same_width() {
        let width: usize = 2;
        let source: &str = "¶¶";
        let output: String = source.pad(width, Alignment::Left, '¨');
        assert_eq!(source, output);
    }

    #[test]
    fn pad_sliced_center_odd() {
        let width: usize = 3;
        let source: &str = "  ¡@£   ";
        let output: String = source.pad(width, Alignment::Center, '¨');
        let expected: &str = "¡@£";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_sliced_center_even() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "§øł0k0";
        assert_eq!(expected, source.pad(width, Alignment::Center, 'x'));
    }

    #[test]
    fn pad_sliced_left() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "1¡§øł0";
        assert_eq!(expected, source.pad(width, Alignment::Left, 'x'));
    }

    #[test]
    fn pad_sliced_right() {
        let width: usize = 6;
        let source: &str = "1¡§øł0k0äツ";
        let expected: &str = "ł0k0äツ";
        assert_eq!(expected, source.pad(width, Alignment::Right, 'x'));
    }

    #[test]
    fn pad_to_buffer_left() {
        let width: usize = 8;
        let source: &str = "Solaire";
        let mut buffer: String = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Left, '\\', &mut buffer);
        let expected: &str = "Solaire\\";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_right() {
        let width: usize = 13;
        let source: &str = "gwyn";
        let mut buffer: String = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Right, 'ö', &mut buffer);
        let expected: &str = "ööööööööögwyn";
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_center() {
        let width: usize = 15;
        let source: &str = "seKiro";
        let mut buffer: String = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Center, 'ツ', &mut buffer);
        let expected: &str = "ツツツツseKiroツツツツツ";
        assert_eq!(expected, buffer);
    }
}

#[cfg(test)]
mod tests_string {
    use super::*;

    #[test]
    fn pad_left() {
        let width: usize = 7;
        let source = String::from("coffee");
        let output: String = source.pad(width, Alignment::Left, ';');
        let expected: &str = "coffee;";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_right() {
        let width: usize = 9;
        let source = String::from("ps5");
        let output: String = source.pad(width, Alignment::Right, '±');
        let expected: &str = "±±±±±±ps5";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center() {
        let width: usize = 31;
        let source = String::from("you are not prepared");
        let output: String = source.pad(width, Alignment::Center, '§');
        let expected: &str = "§§§§§you are not prepared§§§§§§";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_sliced_left() {
        let width: usize = 2;
        let source = String::from("123489700+8471983kbnlajvbroiaye87r687¨ä¨*ÄÂ*ÅWoU)P(FU893y");
        let output: String = source.pad(width, Alignment::Left, '|');
        let expected: &str = "12";
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_sliced_center_odd_odd() {
        let width: usize = 9;
        let source = String::from("1¡§øł0k0äツ+0/");
        let expected: &str = "§øł0k0äツ+";
        assert_eq!(expected, source.pad(width, Alignment::Center, '<'));
    }

    #[test]
    fn pad_sliced_center_odd_even() {
        let width: usize = 3;
        let source = String::from("1234");
        let expected: &str = "123";
        assert_eq!(expected, source.pad(width, Alignment::Center, ' '));
    }

    #[test]
    fn pad_sliced_center_even_even() {
        let width: usize = 6;
        let source = String::from("“ª€][æø0");
        let expected: &str = "ª€][æø";
        assert_eq!(expected, source.pad(width, Alignment::Center, ' '));
    }

    #[test]
    fn pad_sliced_center_even_odd() {
        let width: usize = 6;
        let source = String::from("“ª€][æ🤠ø0");
        let expected: &str = "ª€][æ🤠";
        assert_eq!(expected, source.pad(width, Alignment::Center, ' '));
    }

    #[test]
    fn pad_same_width() {
        let width: usize = 3;
        let source = String::from("123");
        let output: String = source.pad(width, Alignment::Left, '8');
        assert_eq!(source, output);
    }

    #[test]
    fn pad_to_buffer_left() {
        let width: usize = 16;
        let source = String::from("mount gelmir");
        let mut buffer = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Left, '.', &mut buffer);
        let mut expected = String::with_capacity(width);
        expected.push_str("mount gelmir....");
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_right() {
        let width: usize = 16;
        let source = String::from("mount gelmir");
        let mut buffer = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Right, ';', &mut buffer);
        let mut expected = String::with_capacity(width);
        expected.push_str(";;;;mount gelmir");
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_center_even() {
        let width: usize = 16;
        let source = String::from("mount gelmir");
        let mut buffer = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Center, '-', &mut buffer);
        let mut expected = String::with_capacity(width);
        expected.push_str("--mount gelmir--");
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_center_odd() {
        let width: usize = 17;
        let source = String::from("mount gelmir");
        let mut buffer = String::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Center, '-', &mut buffer);
        let mut expected = String::with_capacity(width);
        expected.push_str("--mount gelmir---");
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }
}

#[cfg(test)]
mod tests_vec {
    use super::*;

    #[test]
    fn pad_left() {
        let width: usize = 8;
        let source = Vec::from(&[1u8, 2, 3, 4, 5]);
        let expected = Vec::from(&[1u8, 2, 3, 4, 5, 0, 0, 0]);
        let output = source.pad(width, Alignment::Left, 0u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_right() {
        let width: usize = 14;
        let source = Vec::from(&[1u8, 255, 0, 2, 3]);
        let expected = Vec::from(&[13u8, 13, 13, 13, 13, 13, 13, 13, 13, 1, 255, 0, 2, 3]);
        let output = source.pad(width, Alignment::Right, 13u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center_odd() {
        let width: usize = 5;
        let source = Vec::from(&[1u8, 98]);
        let expected = Vec::from(&[190u8, 1, 98, 190, 190]);
        let output = source.pad(width, Alignment::Center, 190u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center_even() {
        let width: usize = 4;
        let source = Vec::from(&[1u8, 98]);
        let expected = Vec::from(&[190u8, 1, 98, 190]);
        let output = source.pad(width, Alignment::Center, 190u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_to_fit_even() {
        let width: usize = 2;
        let expected = Vec::from(&[1u8, 98]);
        let source = Vec::from(&[190u8, 1, 98, 190]);
        let output = source.pad(width, Alignment::Center, 123u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn slice_to_fit_odd() {
        let width: usize = 1;
        let expected = Vec::from(&[1u8]);
        let source = Vec::from(&[190u8, 1, 98, 190]);
        let output = source.pad(width, Alignment::Center, 123u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected.capacity(), output.capacity());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_same_len() {
        let width: usize = 4;
        let source = Vec::from(&[190u8, 1, 98, 190]);
        let expected = Vec::from(&[190u8, 1, 98, 190]);
        let output = source.pad(width, Alignment::Center, 123u8);
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_to_buffer_left() {
        let width: usize = 5;
        let source = Vec::from(&[1u8, 0, 1]);
        let mut buffer = Vec::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Left, 9u8, &mut buffer);
        let mut expected = Vec::with_capacity(width);
        expected.extend_from_slice(&[1u8, 0, 1, 9, 9]);
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_right() {
        let width: usize = 6;
        let source = Vec::from(&[1u8, 0, 1]);
        let mut buffer = Vec::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Right, 254u8, &mut buffer);
        let mut expected = Vec::with_capacity(width);
        expected.extend_from_slice(&[254u8, 254, 254, 1, 0, 1]);
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_center_odd() {
        let width: usize = 5;
        let source = Vec::from(&[1u8, 0, 1, 2]);
        let mut buffer = Vec::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Center, 148u8, &mut buffer);
        let mut expected = Vec::with_capacity(width);
        expected.extend_from_slice(&[1u8, 0, 1, 2, 148]);
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }

    #[test]
    fn pad_to_buffer_center_even() {
        let width: usize = 6;
        let source = Vec::from(&[1u8, 0, 1, 2]);
        let mut buffer = Vec::with_capacity(width);
        source.pad_to_buffer(width, Alignment::Center, 148u8, &mut buffer);
        let mut expected = Vec::with_capacity(width);
        expected.extend_from_slice(&[148u8, 1, 0, 1, 2, 148]);
        assert_eq!(expected.len(), buffer.len());
        assert_eq!(expected.capacity(), buffer.capacity());
        assert_eq!(expected, buffer);
    }
}

#[cfg(test)]
mod tests_slice {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct DummyStruct {
        a: usize,
        b: usize,
    }

    #[test]
    fn pad_left_struct() {
        let width: usize = 3;
        let source: &[DummyStruct] = &[DummyStruct { a: 2, b: 3 }];
        let output = source.pad(width, Alignment::Left, DummyStruct { a: 4, b: 5 });
        let expected = Vec::from(&[
            DummyStruct { a: 2, b: 3 },
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 4, b: 5 },
        ]);
        assert_eq!(expected.capacity(), output.capacity());
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_right_struct() {
        let width: usize = 3;
        let source: &[DummyStruct] = &[DummyStruct { a: 2, b: 3 }];
        let output = source.pad(width, Alignment::Right, DummyStruct { a: 4, b: 5 });
        let expected = Vec::from(&[
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 2, b: 3 },
        ]);
        assert_eq!(expected.capacity(), output.capacity());
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center_even_struct() {
        let width: usize = 3;
        let source: &[DummyStruct] = &[DummyStruct { a: 2, b: 3 }];
        let output = source.pad(width, Alignment::Center, DummyStruct { a: 4, b: 5 });
        let expected = Vec::from(&[
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 2, b: 3 },
            DummyStruct { a: 4, b: 5 },
        ]);
        assert_eq!(expected.capacity(), output.capacity());
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }

    #[test]
    fn pad_center_odd_struct() {
        let width: usize = 4;
        let source: &[DummyStruct] = &[DummyStruct { a: 2, b: 3 }];
        let output = source.pad(width, Alignment::Center, DummyStruct { a: 4, b: 5 });
        let expected = Vec::from(&[
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 2, b: 3 },
            DummyStruct { a: 4, b: 5 },
            DummyStruct { a: 4, b: 5 },
        ]);
        assert_eq!(expected.capacity(), output.capacity());
        assert_eq!(expected.len(), output.len());
        assert_eq!(expected, output);
    }
}
