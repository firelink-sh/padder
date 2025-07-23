use crate::alignment::Alignment;

/// A trait representing a mutable, width-aware data buffer that can be padded (and truncated).
///
/// Types implementing [`MutableSource`] expose the method [`pad`] for resizing themselves to a specific width,
/// either by trimming excess data or inserting padding symbols on one or both sides of the buffer.
/// This is useful for formatting structures like [`String`]s or [`Vec`]s for display or layout.
///
/// # Associated Types
/// - `Symbol`: the element used for padding (e.g., `char`, `u8`, or anything that implements [`Copy`]).
/// - `Buffer`: the underying mutable buffer type.
///
/// # Optional unsafe optimization
/// If compiled with the `enable_unsafe` feature flag, implementations will utilize `unsafe` code
/// for improved performance (through manual buffer length adjustments and unchecked memory writes).
///
/// [`pad`]: MutableSource::pad
pub trait MutableSource {
    type Symbol;
    type Buffer;

    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol);
}

impl MutableSource for &mut String {
    type Symbol = char;
    type Buffer = Self;

    #[cfg(not(feature = "enable_unsafe"))]
    /// Pads or truncates the string to match the specified width with a given alignment.
    ///
    /// If the string is longer than `width` (in utf8 chars), it will be truncated according to the `mode`:
    /// - [`Alignment::Left`]: truncates from the right.
    /// - [`Alignment::Right`]: truncates from the left.
    /// - [`Alignment::Center`]: trims equally from both ends (extra char trimmed from the left if number of chars to trim is odd).
    ///
    /// If the buffer is shorter than `width`, it will be padded using the specified `symbol`:
    /// - Padding is distributed based on alignment: left, right, or center (extra symbol on the right if number of chars to pad is odd).
    /// - The implementation performs two temporary allocations to construct the padded version (much more efficient than performing repeated [`insert()`] calls).
    ///
    /// The result replaces the original string.
    ///
    /// # Examples
    /// ```
    /// use padder::*;
    ///
    /// let mut s = String::from("Visa Vid Vindens Ängar");
    /// let width: usize = 25;
    /// (&mut s).pad(width, Alignment::Center, '¡');  // "¡Visa Vid Vindens Ängar¡¡"
    ///
    /// assert_eq!(25, s.chars().count());
    /// assert_eq!(23 + 2 * 3, s.capacity());  // '¡' = 2 bytes & 'Ä' = 2 bytes
    /// ```
    /// [`insert()`]: String::insert()
    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol) {
        let n_chars_original: usize = self.chars().count();
        if width < n_chars_original {
            match mode {
                Alignment::Left => {
                    let byte_offset_trunc: usize = self
                        .char_indices()
                        .nth(width)
                        .map(|(byte_offset, _)| byte_offset)
                        .expect("the String did not contain enough chars!");
                    self.truncate(byte_offset_trunc);
                }
                Alignment::Right => {
                    let byte_offset_drain = self
                        .char_indices()
                        .rev()
                        .nth(width - 1)
                        .map(|(byte_offset, _)| byte_offset)
                        .expect("the String did not contain enough chars!");
                    self.drain(..byte_offset_drain);
                }
                Alignment::Center => {
                    let st_idx: usize = (n_chars_original - width) / 2;
                    let ed_idx: usize = st_idx + width;

                    let mut st_byte: usize = 0;
                    let mut ed_byte: usize = self.len();

                    for (idx, (byte_offset, _)) in self.char_indices().enumerate() {
                        if idx == st_idx {
                            st_byte = byte_offset;
                        }
                        if idx == ed_idx {
                            ed_byte = byte_offset;
                            break;
                        }
                    }

                    self.drain(..st_byte);
                    self.truncate(ed_byte - st_byte);
                }
            };
            self.shrink_to_fit();
            return;
        }

        let n_chars_diff: usize = width - n_chars_original;
        if n_chars_diff == 0 {
            return;
        }

        let pads = mode.pads(n_chars_diff);
        let mut new_s: String = std::iter::repeat_n(symbol, pads.left()).collect();

        new_s.push_str(self);
        new_s.push_str(&std::iter::repeat_n(symbol, pads.right()).collect::<String>());

        new_s.shrink_to_fit();
        **self = new_s;
    }

    #[cfg(feature = "enable_unsafe")]
    /// Pads or truncates the string to match the specified width with a given alignment.
    ///
    /// If the string is longer than `width` (in utf8 chars), it will be truncated according to the `mode`:
    /// - [`Alignment::Left`]: truncates from the right.
    /// - [`Alignment::Right`]: truncates from the left.
    /// - [`Alignment::Center`]: trims equally from both ends (extra char trimmed from the left if number of chars to trim is odd).
    ///
    /// If the buffer is shorter than `width`, it will be padded using the specified `symbol`:
    /// - Padding is distributed based on alignment: left, right, or center (extra symbol on the right if number of chars to pad is odd).
    /// - This implementation performs no heap allocations to construct the padded version (but introduces `unsafe` code).
    ///
    /// The result replaces the original string.
    ///
    /// # Safety
    /// This implementation makes use of the [`set_len()`] and [`copy_within()`] methods to directly
    /// modify the contents of the String buffer without having to perform any extra allocations.
    ///
    /// This greatly improves performance when padding large strings, truncating performance should be unchanged.
    ///
    /// # Examples
    /// ```
    /// use padder::*;
    ///
    /// let mut s = String::from("sackboy");
    /// let width: usize = 11;
    /// (&mut s).pad(width, Alignment::Right, '-');  // "----sackboy
    ///
    /// let mut expected = String::from("----sackboy");
    /// expected.shrink_to_fit();
    ///
    /// assert_eq!(11, s.chars().count());
    /// assert_eq!(11, s.capacity());
    /// assert_eq!(expected.len(), s.len());
    /// assert_eq!(expected, s);
    /// ```
    /// [`set_len()`]: Vec::set_len()
    /// [`copy_within()`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_within
    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol) {
        let n_chars_original: usize = self.chars().count();
        let n_bytes_original: usize = self.len();

        if width < n_chars_original {
            match mode {
                Alignment::Left => {
                    let byte_offset_trunc: usize = self
                        .char_indices()
                        .nth(width)
                        .map(|(byte_offset, _)| byte_offset)
                        .expect("the String did not contain enough chars!");
                    self.truncate(byte_offset_trunc);
                }
                Alignment::Right => {
                    let byte_offset_drain: usize = self
                        .char_indices()
                        .rev()
                        .nth(width - 1)
                        .map(|(byte_offset, _)| byte_offset)
                        .expect("the String did not contain enough chars!");
                    self.drain(..byte_offset_drain);
                }
                Alignment::Center => {
                    let st_idx: usize = (n_chars_original - width) / 2;
                    let ed_idx: usize = st_idx + width;

                    let mut st_byte: usize = 0;
                    let mut ed_byte: usize = self.len();

                    for (idx, (byte_offset, _)) in self.char_indices().enumerate() {
                        if idx == st_idx {
                            st_byte = byte_offset;
                            continue;
                        }
                        if idx == ed_idx {
                            ed_byte = byte_offset;
                            break;
                        }
                    }

                    self.drain(..st_byte);
                    self.truncate(ed_byte - st_byte);
                }
            }
            self.shrink_to_fit();
            return;
        }

        let n_chars_diff: usize = width - n_chars_original;
        if n_chars_diff == 0 {
            return;
        }

        let n_bytes_symbol: usize = symbol.len_utf8();
        let n_bytes_diff: usize = n_chars_diff * n_bytes_symbol;

        let pads = mode.pads(n_chars_diff);
        let n_bytes_l_pad = pads.left() * n_bytes_symbol;
        let n_bytes_r_pad = pads.right() * n_bytes_symbol;

        self.reserve_exact(n_bytes_diff);

        unsafe {
            let buf: &mut Vec<u8> = self.as_mut_vec();
            buf.set_len(n_bytes_original + n_bytes_diff);
            buf.copy_within(..(n_bytes_original + n_bytes_r_pad), n_bytes_l_pad);

            let mut byte_offset_l: usize = 0;
            for _ in 0..pads.left() {
                byte_offset_l += symbol.encode_utf8(&mut buf[byte_offset_l..]).len();
            }

            let mut byte_offset_r: usize = buf.len();
            for _ in 0..pads.right() {
                byte_offset_r -= n_bytes_symbol;
                symbol.encode_utf8(&mut buf[byte_offset_r..]);
            }
        }
    }
}

impl<T> MutableSource for &mut Vec<T>
where
    T: Copy + Sized,
{
    type Symbol = T;
    type Buffer = Self;

    /// Pads or truncates the buffer to match the specified width with a given alignment.
    ///
    /// If the buffer is longer than `width` (in bytes), it will be truncated according to the `mode`:
    /// - [`Alignment::Left`]: truncates from the right.
    /// - [`Alignment::Right`]: truncates from the left.
    /// - [`Alignment::Center`]: trims equally from both ends (extra byte trimmed from the left if number of bytes to trim is odd).
    ///
    /// If the buffer is shorter than `width`, it will be padded using the specified `symbol`:
    /// - Padding is distributed based on alignment: left, right, or center (extra symbol on the right if number of bytes to pad is odd).
    /// - The implementation performs two temporary allocations to construct the padded version (much more efficient than performing repeated [`insert()`] calls).
    ///
    /// The result replaces the original buffer.
    ///
    /// # Examples
    /// ```
    /// use padder::*;
    ///
    /// let mut v: Vec<char> = vec!['y', 'o', 'o'];
    /// let width: usize = 7;
    /// (&mut v).pad(width, Alignment::Left, '!');  // ['y', 'o', 'o', '!', '!', '!', '!']
    ///
    /// #[derive(Debug, Default, Copy, Clone, PartialEq)]
    /// struct DummyStruct {
    ///     a: usize,
    ///     b: bool,
    /// }
    ///
    /// let mut v: Vec<DummyStruct> = Vec::from(&[
    ///     DummyStruct { a: 3, b: false },
    ///     DummyStruct { a: 2, b: true },
    ///     DummyStruct { a: 1, b: false },
    ///     DummyStruct { a: 15, b: false },
    /// ]);
    /// v.shrink_to_fit();
    ///
    /// let width: usize = 7;
    /// (&mut v).pad(width, Alignment::Right, DummyStruct { a: 1337, b: true });
    ///
    /// let mut expected: Vec<DummyStruct> = Vec::from(&[
    ///     DummyStruct { a: 1337, b: true },
    ///     DummyStruct { a: 1337, b: true },
    ///     DummyStruct { a: 1337, b: true },
    ///     DummyStruct { a: 3, b: false },
    ///     DummyStruct { a: 2, b: true },
    ///     DummyStruct { a: 1, b: false },
    ///     DummyStruct { a: 15, b: false },
    /// ]);
    /// expected.shrink_to_fit();
    ///
    /// assert_eq!(expected.len(), v.len());
    /// assert_eq!(expected.capacity(), v.capacity());
    /// assert_eq!(expected, v);
    ///
    /// // we can modify the original vec again!
    /// (&mut v).pad(2, Alignment::Left, DummyStruct::default());  // the pad symbol doesn't matter when we truncate
    ///
    /// expected.truncate(expected.len().saturating_sub(width - 2));
    ///
    /// assert_eq!(expected, v);
    /// ```
    ///
    /// [`insert()`]: Vec::insert()
    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol) {
        if width < self.len() {
            match mode {
                Alignment::Left => {
                    self.truncate(width);
                }
                Alignment::Right => {
                    let byte_offset_drain: usize = self.len() - width;
                    self.drain(..byte_offset_drain);
                }
                Alignment::Center => {
                    let byte_offset_drain: usize = (self.len() - width) / 2;
                    self.drain(..byte_offset_drain);
                    self.truncate(width);
                }
            }
            self.shrink_to_fit();
            return;
        }

        let n_bytes_diff: usize = width - self.len();
        if n_bytes_diff == 0 {
            return;
        }

        let pads = mode.pads(n_bytes_diff);
        let mut new_v: Vec<T> = std::iter::repeat_n(symbol, pads.left()).collect();

        new_v.extend_from_slice(self);
        new_v.extend_from_slice(&std::iter::repeat_n(symbol, pads.right()).collect::<Vec<T>>());

        new_v.shrink_to_fit();
        **self = new_v;
    }
}

#[cfg(test)]
mod tests_string {
    use super::*;

    #[test]
    fn pad_left() {
        let width: usize = 17;
        let mut source = String::from("Vilhelm Moberg");
        (&mut source).pad(width, Alignment::Left, '@');
        let expected = String::from("Vilhelm Moberg@@@");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_right() {
        let width: usize = 12;
        let mut source = String::from("rocketTT");
        (&mut source).pad(width, Alignment::Right, '🚀');
        let expected = String::from("🚀🚀🚀🚀rocketTT");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_center_odd() {
        let width: usize = 8;
        let mut source = String::from("plant");
        (&mut source).pad(width, Alignment::Center, 'き');
        let expected = String::from("きplantきき");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_center_even() {
        let width: usize = 16;
        let mut source = String::from("實real實");
        (&mut source).pad(width, Alignment::Center, '實');
        let expected = String::from("實實實實實實real實實實實實實");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_left() {
        let width: usize = 3;
        let mut source = String::from("實real實");
        (&mut source).pad(width, Alignment::Left, '實');
        let expected = String::from("實re");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_right() {
        let width: usize = 5;
        let mut source = String::from("實real實");
        (&mut source).pad(width, Alignment::Right, '實');
        let expected = String::from("real實");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_center_odd() {
        let width: usize = 6;
        let mut source = String::from("實vamos實carlito實");
        (&mut source).pad(width, Alignment::Center, '實');
        let expected = String::from("os實car");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_center_even() {
        let width: usize = 7;
        let mut source = String::from("實vamos實carlito實");
        (&mut source).pad(width, Alignment::Center, '實');
        let expected = String::from("os實carl");
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }
}

#[cfg(test)]
mod tests_vec {
    use super::*;

    #[test]
    fn pad_left() {
        let width: usize = 4;
        let mut source: Vec<u32> = Vec::from(&[1u32, 2, 3]);
        (&mut source).pad(width, Alignment::Left, 1337);
        let expected: Vec<u32> = Vec::from(&[1u32, 2, 3, 1337]);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_right() {
        let width: usize = 6;
        let mut source: Vec<i32> = Vec::from(&[1i32, 2, 3]);
        (&mut source).pad(width, Alignment::Right, -1998);
        let expected: Vec<i32> = Vec::from(&[-1998i32, -1998, -1998, 1, 2, 3]);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_center_odd() {
        let width: usize = 6;
        let mut source: Vec<char> = Vec::from(&['😺', '2', '¡']);
        (&mut source).pad(width, Alignment::Center, '🐛');
        let expected: Vec<char> = Vec::from(&['🐛', '😺', '2', '¡', '🐛', '🐛']);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn pad_center_even() {
        let width: usize = 7;
        let mut source: Vec<char> = Vec::from(&['😺', '2', '¡']);
        (&mut source).pad(width, Alignment::Center, '🐛');
        let expected: Vec<char> = Vec::from(&['🐛', '🐛', '😺', '2', '¡', '🐛', '🐛']);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_left() {
        let width: usize = 2;
        let mut source: Vec<char> = Vec::from(&['😺', '2', '¡']);
        (&mut source).pad(width, Alignment::Left, ' ');
        let expected: Vec<char> = Vec::from(&['😺', '2']);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct DummyStruct {
        a: bool,
    }

    #[test]
    fn truncate_right() {
        let width: usize = 3;
        let mut source: Vec<DummyStruct> = Vec::from(&[
            DummyStruct { a: true },
            DummyStruct { a: false },
            DummyStruct { a: false },
            DummyStruct { a: false },
            DummyStruct { a: true },
        ]);
        (&mut source).pad(width, Alignment::Right, DummyStruct { a: false });
        let expected: Vec<DummyStruct> = Vec::from(&[
            DummyStruct { a: false },
            DummyStruct { a: false },
            DummyStruct { a: true },
        ]);

        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_center_odd() {
        let width: usize = 4;
        let mut source: Vec<&str> = Vec::from(&[
            "yooo",
            "radahn",
            "this is a longer string hihi",
            "beethoven",
            "mozart",
            "chopin",
            "rachmaninoff",
        ]);
        (&mut source).pad(width, Alignment::Center, "padded");
        let expected: Vec<&str> = Vec::from(&[
            "radahn",
            "this is a longer string hihi",
            "beethoven",
            "mozart",
        ]);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }

    #[test]
    fn truncate_center_even() {
        let width: usize = 5;
        let mut source: Vec<&str> = Vec::from(&[
            "yooo",
            "radahn",
            "this is a longer string hihi",
            "beethoven",
            "mozart",
            "chopin",
            "rachmaninoff",
        ]);
        (&mut source).pad(width, Alignment::Center, "padded");
        let expected: Vec<&str> = Vec::from(&[
            "radahn",
            "this is a longer string hihi",
            "beethoven",
            "mozart",
            "chopin",
        ]);
        assert_eq!(expected.capacity(), source.capacity());
        assert_eq!(expected.len(), source.len());
        assert_eq!(expected, source);
    }
}
