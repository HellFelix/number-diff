mod functions;
pub use functions::Elementary;

mod differentiation;

mod integration;
pub use integration::integrate;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;
