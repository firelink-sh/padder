/// Represents padding as a pair of `(left, right)` counts.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pads(pub usize, pub usize);

impl Pads {
    /// Get the number of padding symbols that should be prepended to the buffer.
    pub fn left(&self) -> usize {
        self.0
    }

    /// Get the number of padding symbols that should be appended to the buffer.
    pub fn right(&self) -> usize {
        self.1
    }
}

/// Specifies the alignment strategy to use when padding or truncating a buffer.
///
/// - `Left`: aligns content to the left, padding goes to the right.
/// - `Right`: aligns content to the right, padding goes to the left.
/// - `Center`: distributes padding equally on both sides (extra symbol goes to the right of number of symbols to padd is odd).
///
/// # Examples
/// ```
/// use padder::{Alignment, Pads};
///
/// let l = Alignment::Left;
/// assert_eq!(Pads(0, 4), l.pads(4));
///
/// let r = Alignment::Right;
/// assert_eq!(Pads(52, 0), r.pads(52));
///
/// let c = Alignment::Center;
/// assert_eq!(Pads(5, 6), c.pads(11));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    #[default]
    Right,
    Center,
}

impl Alignment {
    /// Returns a [`Pads`] struct representing how `n` padding symbols should be distributed
    /// according to the alignment mode.
    ///
    /// See [`Alignment`] for more details.
    pub fn pads(&self, n: usize) -> Pads {
        match self {
            Self::Left => Pads(0, n),
            Self::Right => Pads(n, 0),
            Self::Center => Pads(n / 2, n - n / 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_left() {
        let a = Alignment::Left;
        assert_eq!(Pads(0, 5), a.pads(5));
    }

    #[test]
    fn alignment_right() {
        let a = Alignment::Right;
        assert_eq!(Pads(123456, 0), a.pads(123456));
    }

    #[test]
    fn alignment_center() {
        let a = Alignment::Center;
        assert_eq!(Pads(4, 5), a.pads(9));
    }

    #[test]
    fn alignment_default() {
        let a = Alignment::default();
        let p = a.pads(9149);
        let expected = Pads(9149, 0);
        assert_eq!(Alignment::Right, a);
        assert_eq!(expected, p);
        assert_eq!(expected.left(), p.left());
        assert_eq!(0, p.right());
    }
}
