use crate::{Elementary, Error, Func, Function, Round};

/// types that implement the [Integrate](crate::Integrate) trait can safely be integrated within
/// the domain ℝ.
pub trait Integrate {
    /// Method will return an instance of [Integral](crate::Integral) that can be taylored to
    /// specific usecases.
    ///
    /// Example:
    /// ```rust
    /// let function = Function::from("sin(x)");
    ///
    /// let mut integral = function.integrate();

    /// // specify bounds and precision for the integral
    /// integral
    ///     .set_lower_bound(0.)
    ///     .set_upper_bound(PI / 2.)
    ///     .set_precision(20000);
    ///
    /// // evaluate the integral
    /// let value = integral.evaluate().unwrap();
    /// // note that the value of the evaluated integral must be unwrapped if using the `integrate()`
    /// // method because the method cannot guarantee that bounds have been set at the point of
    /// // evaluating. The evaluate_integral() method which is implemented for any instance with the
    /// // Integrate trait is safer and is guaranteed to yield a valid result.
    ///
    /// // round the value to 10 decimal places
    /// value.round_to(10);
    ///
    /// assert_eq!(value, 1.);
    /// ```
    /// Also note that if the precision is not specified, it will default to 1000.
    fn integrate(&self) -> Integral;

    /// Method will return the value of the definite integral of the function evaluated from the
    /// provided lower and upper bound.
    ///
    /// Example:
    /// ```
    /// let function = Function::from("cos(x)");
    ///
    /// // the evaluate_integral() method will automatically round the result to five decimal points.
    /// // This is because higher precision cannot be guaranteed with using the standard precision set
    /// // for the method. Provided that the function is defined for all values between the lower and
    /// // upper bounds, the method will always return a valid result.
    /// let value = function.evaluate_integral(0., PI);
    ///
    /// assert_eq!(value, 0.);
    /// ```
    fn evaluate_integral(&self, lower_bound: f64, upper_bound: f64) -> f64;
}

const STANDARD_PRECISION: usize = 1000;

pub struct Integral {
    function: Func,
    lower_bound: Option<f64>,
    upper_bound: Option<f64>,
    precision: usize,
}

impl Integral {
    pub fn vacant(function: Func) -> Self {
        Self {
            function,
            lower_bound: None,
            upper_bound: None,
            precision: STANDARD_PRECISION,
        }
    }

    pub fn set_lower_bound(&mut self, lower_bound: f64) -> &mut Self {
        self.lower_bound = Some(lower_bound);
        self
    }

    pub fn set_upper_bound(&mut self, upper_bound: f64) -> &mut Self {
        self.upper_bound = Some(upper_bound);
        self
    }

    pub fn set_precision(&mut self, precision: usize) -> &mut Self {
        self.precision = precision;
        self
    }

    pub fn evaluate(&self) -> Result<f64, Error> {
        if let (Some(lower_bound), Some(upper_bound)) = (self.lower_bound, self.upper_bound) {
            Ok(simpsons_rule(
                &self.function,
                lower_bound,
                upper_bound,
                self.precision,
            ))
        } else {
            Err(Error::InternalError(String::from(
                "Bounds of integration must be set in order to evaluate the integral",
            )))
        }
    }
}

/// See [Integrate](crate::Integrate) for usage and examples.
impl Integrate for Elementary {
    fn integrate(&self) -> Integral {
        Integral::vacant(self.clone().call())
    }
    /// Evaluating the integral gives a value of the integral with eight decimal places
    fn evaluate_integral(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        unsafe {
            let mut value = self
                .integrate()
                .set_lower_bound(lower_bound)
                .set_upper_bound(upper_bound)
                .evaluate()
                .unwrap_unchecked(); // this unwrap will never fail because the upper and lower bounds
                                     // will always be set
            value.with_significant_figures(5)
        }
    }
}

/// See [Integrate](crate::Integrate) for usage and examples.
impl Integrate for Function {
    fn integrate(&self) -> Integral {
        self.elementary().integrate()
    }
    fn evaluate_integral(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        self.elementary()
            .evaluate_integral(lower_bound, upper_bound)
    }
}

fn simpsons_rule(funciton: &Func, lower_bound: f64, upper_bound: f64, precision: usize) -> f64 {
    // note that n must be an even number for Simpson's rule to work
    let n = precision * 2;
    let dx = (upper_bound - lower_bound) / n as f64;
    let mut sum: f64 = (1..n)
        .map(|x| {
            if x % 2 == 0 {
                2. * funciton(lower_bound + x as f64 * dx)
            } else {
                4. * funciton(lower_bound + x as f64 * dx)
            }
        })
        .sum();
    sum += funciton(lower_bound) + funciton(upper_bound);

    sum * dx / 3.
}
