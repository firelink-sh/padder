use crate::alignment::Alignment;

/// Implementation of a mutable `source` to pad.
pub trait MutableSource {
    type Symbol;
    type Buffer;

    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol);
}

impl MutableSource for &mut String {
    type Symbol = char;
    type Buffer = Self;

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
                    self.drain(0..byte_offset_drain);
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

                    self.drain(0..st_byte);
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
        // This performs two new allocations - but much faster than `insert()` on
        // large strings, and only marginally slower than using `unsafe` on large strings!.
        let mut new_s: String = std::iter::repeat_n(symbol, pads.left()).collect();

        new_s.push_str(self);
        new_s.push_str(&std::iter::repeat_n(symbol, pads.right()).collect::<String>());

        new_s.shrink_to_fit();
        **self = new_s;
    }
}

impl<T> MutableSource for &mut Vec<T>
where
    T: Copy + Sized,
{
    type Symbol = T;
    type Buffer = Self;

    fn pad(&mut self, width: usize, mode: Alignment, symbol: Self::Symbol) {
        if width < self.len() {
            match mode {
                Alignment::Left => {
                    self.truncate(width);
                }
                Alignment::Right => {
                    let byte_offset_drain: usize = self.len() - width;
                    self.drain(0..byte_offset_drain);
                }
                Alignment::Center => {
                    let byte_offset_drain: usize = (self.len() - width) / 2;
                    self.drain(0..byte_offset_drain);
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
        // This performs two new allocations - but should be faster than insert?
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
        let mut source = String::from("Wilhelm Moberg");
        (&mut source).pad(width, Alignment::Left, '@');
        let expected = String::from("Wilhelm Moberg@@@");
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
