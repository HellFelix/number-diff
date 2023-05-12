use crate::functions::calc::Elementary;
pub use crate::functions::calc::{acos, asin, atan, cos, sin, tan};

pub use crate::Elementary::X;

pub use crate::functions::differentiation;

pub use crate::functions::integration::integrate;

pub use crate::functions::calc::Function;

pub fn test_parse<'a>(value: &'a String) -> Elementary {
    Elementary::test_from(&value)
}
