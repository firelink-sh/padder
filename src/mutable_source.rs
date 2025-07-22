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
        let n_chars_current: usize = self.chars().count();
        if width < n_chars_current {
            todo!()
        }

        let n_chars_diff: usize = width - n_chars_current;
        if n_chars_diff == 0 {
            return;
        }

        let n_bytes_symbol: usize = symbol.len_utf8();
        let n_bytes_original: usize = self.len();
        let n_bytes_diff: usize = n_chars_diff * n_bytes_symbol;
        self.reserve_exact(n_bytes_diff);

        let pads = mode.pads(n_chars_diff);
        let n_bytes_left_pad = pads.left() * n_bytes_symbol;
        let n_bytes_right_pad = pads.right() * n_bytes_symbol;

        for _ in 0..pads.right() {
            self.push(symbol);
        }

        unsafe {
            let buf = self.as_mut_vec();
            // Need to manually update the length of the buffer preemptively, because
            // the `buf.copy_within()` method checks its own length to validate that
            // we are allowed to move contents within the slice.
            buf.set_len(n_bytes_original + n_bytes_diff);

            // Shift to the right using `memmove` in the allocated buffer.
            buf.copy_within(0..(n_bytes_original + n_bytes_right_pad), n_bytes_left_pad);

            let mut byte_offset: usize = 0;
            for _ in 0..pads.left() {
                byte_offset += symbol.encode_utf8(&mut buf[byte_offset..]).len();
            }
        }
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
}
