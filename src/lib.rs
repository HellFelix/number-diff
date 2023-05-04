#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![cfg_attr(feature = "nightly", feature(tuple_trait))]


mod functions;
pub use crate::functions::calc::Elementary;

pub use crate::functions::differentiation;

pub use crate::functions::integration::integrate;

pub use crate::functions::calc::Function;
mod utils;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;
