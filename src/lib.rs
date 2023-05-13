#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![cfg_attr(feature = "nightly", feature(tuple_trait))]

mod functions;
use functions::calc::Elementary;

mod utils;
pub use utils::include::*;

type Func = Box<dyn Fn(f64) -> f64 + 'static>;
