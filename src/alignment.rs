#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pads(usize, usize);

impl Pads {
    pub fn left(&self) -> usize {
        self.0
    }

    pub fn right(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    #[default]
    Right,
    Center,
}

impl Alignment {
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
