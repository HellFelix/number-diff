use std::sync::Arc;

use crate::{
    Elementary::{self, *},
    Error, Factorial, Function,
};

#[derive(Debug, Clone)]
/// [SeriesExpansion](crate::SeriesExpansion) is an abstraction of the series expansion created when using
pub enum SeriesExpansion {
    /// A Taylor series expansion centered around 0.
    MacLaurin(Elementary),
    /// An approximation of an analytic function centered around some value using a polynomial.
    /// See [this article](https://en.wikipedia.org/wiki/Taylor_series) for further information.
    Taylor(Elementary),
    /// An approximation of a periodic function into a sum of trigonometric functions.
    /// See [this article](https://en.wikipedia.org/wiki/Fourier_series) for further information.
    Fourier(Elementary),
}
impl SeriesExpansion {
    /// Returns a [Function](crate::Function) instance from the provided
    /// [SeriesExpansion](crate::SeriesExpansion) instance, consuming it in the process.
    pub fn get_function(self) -> Function {
        match self {
            Self::MacLaurin(elem) => Function::from(elem),
            Self::Taylor(elem) => Function::from(elem),
            Self::Fourier(elem) => Function::from(elem),
        }
    }

    /// Returns a [Elementary](crate::Elementary) instance from the provided
    /// [SeriesExpansion](crate::SeriesExpansion) instance, consuming it in the process.
    pub fn get_elementary(self) -> Elementary {
        match self {
            Self::MacLaurin(elem) => elem,
            Self::Taylor(elem) => elem,
            Self::Fourier(elem) => elem,
        }
    }
}

impl Elementary {
    pub fn expand_maclaurin(&self, order: u8) -> Result<SeriesExpansion, Error> {
        let series = self.expand_taylor(order, 0.)?;

        if let SeriesExpansion::Taylor(res) = series {
            Ok(SeriesExpansion::MacLaurin(res))
        } else {
            unreachable!()
        }
    }

    pub fn expand_taylor(&self, order: u8, centre: f64) -> Result<SeriesExpansion, Error> {
        let mut terms: Vec<Elementary> = Vec::new();

        let mut current_derivative = self.clone();

        let first_term = current_derivative.clone().call()(centre);

        terms.push(Con(first_term));

        for i in 1..=order {
            current_derivative = current_derivative.derivative_unsimplified();

            let ith_term = Pow(Arc::new(X - centre), Arc::new(Con(i as f64)))
                * current_derivative.clone().call()(centre)
                / ((i as usize).factorial() as f64);

            terms.push(ith_term);
        }

        let mut res = Con(0.);

        for term in terms {
            res += term;
        }

        res = res.simplify()?;

        // TODO: check the result against lagrange_error_bound to make sure the maximum error is
        // within the bound

        Ok(SeriesExpansion::Taylor(res))
    }
}
