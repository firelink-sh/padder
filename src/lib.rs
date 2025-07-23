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
