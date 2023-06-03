use std::sync::Arc;

use crate::{
    Elementary::{self, *},
    Error, Function,
};

use super::utils::factorial;

#[derive(Debug, Clone)]
pub enum SeriesExpansion {
    MacLaurin(Elementary),
    Taylor(Elementary),
    // Fourier,
    // Dirichlet,
}
impl SeriesExpansion {
    pub fn get_function(&self) -> Function {
        match self {
            SeriesExpansion::MacLaurin(elem) => Function::from(elem.clone()),
            SeriesExpansion::Taylor(elem) => Function::from(elem.clone()),
        }
    }

    pub fn get_elementary(&self) -> Elementary {
        match self {
            SeriesExpansion::MacLaurin(elem) => elem.clone(),
            SeriesExpansion::Taylor(elem) => elem.clone(),
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

    pub fn expand_taylor(&self, order: u8, a: f64) -> Result<SeriesExpansion, Error> {
        let mut terms: Vec<Elementary> = Vec::new();

        let mut current_derivative = self.clone();
        let first_term = current_derivative.clone().call()(a);
        terms.push(Con(first_term));
        for i in 1..=order {
            current_derivative = current_derivative.differentiate();
            let ith_term = Pow(Arc::new(X - a), Arc::new(Con(i as f64)))
                * current_derivative.clone().call()(a)
                / (factorial(i) as f64);

            terms.push(ith_term);
        }

        let mut res = Con(0.);

        for term in terms {
            res += term;
        }

        // TODO: check the result against lagrange_error_bound to make sure the maximum error is
        // within the bound

        Ok(SeriesExpansion::Taylor(res))
    }
}

// returns the greatest possible remainder of a Taylor series expansion.
fn lagrange_error_bound(function: Elementary, order: u8, a: f64) -> f64 {
    todo!()
}
