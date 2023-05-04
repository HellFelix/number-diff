#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod functions;
pub use crate::functions::calc::Elementary;

pub use crate::functions::differentiation;

pub use crate::functions::integration::integrate;

pub use crate::functions::calc::Function;
mod utils;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;
