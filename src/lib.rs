mod functions;
pub use crate::functions::calc::Elementary;

pub use crate::functions::differentiation;

pub use crate::functions::integration::integrate;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;
