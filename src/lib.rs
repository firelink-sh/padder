//!
//! Highly efficient data formatting and padding crate for Rust.
//!
//! ...
//!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::ops::Index;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Alignment {
    Left,
    #[default]
    Right,
    Center,
}

pub trait Source: Sized {
    type Symbol;
    type Buffer;
    type Slice: ?Sized;
    type Output;

    fn len(&self) -> usize;

    fn slice_to_fit(&self, width: usize, mode: Alignment) -> &Self::Slice
    where
        for<'a> &'a Self: Index<Range<usize>, Output = Self::Slice>,
        for<'a> &'a Self: Index<RangeFrom<usize>, Output = Self::Slice>,
        for<'a> &'a Self: Index<RangeTo<usize>, Output = Self::Slice>,
        for<'a> &'a Self: Index<RangeFull, Output = Self::Slice>,
    {
        match mode {
            Alignment::Left => &self[0..width],
            Alignment::Right => &self[(self.len() - width)..],
            Alignment::Center => {
                let st: usize = self.len() / 2 - width / 2;
                let ed: usize = self.len() / 2 + width / 2 + width % 2;
                &self[st..ed]
            },
        }
    }

    fn pad(&self, width: usize, mode: Alignment, symbol: Self::Symbol) -> Self::Output;

    fn pad_and_push_to_buffer(
        &self,
        width: usize,
        mode: Alignment,
        symbol: Self::Symbol,
        buffer: &mut Self::Buffer,
    );
}
