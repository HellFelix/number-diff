use std::{collections::HashMap, sync::Arc};

use crate::{
    factorial,
    Elementary::{self, *},
    Error,
};

use itertools::Itertools;

// Genreal idea goes as follows:
//
// 1. Figure out if the polynomial is a rational expression or if it's a regular polynomail
//
// For regular polynomials:
// 1. split the polynomial apart
// 2. expand each term
// 3. convert each term into the form ax^b
// 4. group terms together based on their power of the independent variable
// 5. calculate coefficients
// 6. put together
//
// For rational polynomials:
// 1. simplify numerator and denomenator seperatly using the method for regular polynomials
// 2. perform polynomial division.

pub fn simplify_polynomial(polynomial: Elementary) -> Result<Elementary, Error> {
    // figure out if the polynomial is regular or rational
    if is_rational(polynomial.clone()) {
        todo!()
    } else {
        // we have a regular polynomial

        // split the polynomial and get the individual terms
        let terms = get_terms(polynomial.clone())?;

        // expand each term
        let mut expanded_terms: Vec<Elementary> = Vec::new();
        for term in terms {
            for t in expand_term(&term)? {
                // expanded_terms.push(simplify_polynomial(t)?);
                expanded_terms.push(t);
            }
        }

        // convert each item to Mul(Con(_), Pow(X, Con(INTEGER)))
        let mut converted_terms: Vec<Elementary> = Vec::new();
        for term in expanded_terms {
            converted_terms.push(convert_term(term)?);
        }

        // group the terms together based on their x-value
        let groups = group_together(converted_terms)?;

        let keys: Vec<&i128> = groups.keys().sorted().collect();

        // initialize the resulting polynomial
        let mut simplified_polynomial =
            get_polynomial_chunk(*groups.get(keys[0]).unwrap(), *keys[0]);

        for i in 1..keys.len() {
            simplified_polynomial += get_polynomial_chunk(*groups.get(keys[i]).unwrap(), *keys[i]);
        }

        Ok(simplified_polynomial)
    }
}

fn is_rational(polynomial: Elementary) -> bool {
    if let Div(_, pol2) = polynomial {
        if !(*pol2).is_constant() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn get_polynomial_chunk(coefficient: f64, degree: i128) -> Elementary {
    if coefficient == 0. {
        Con(0.)
    } else if degree == 0 {
        Con(coefficient.clone())
    } else if coefficient == 1. {
        if degree == 1 {
            X
        } else {
            Pow(X.into(), Con(degree as f64).into())
        }
    } else {
        Mul(
            Con(coefficient.clone()).into(),
            Pow(X.into(), Con(degree as f64).into()).into(),
        )
    }
}

fn get_terms(polynomial: Elementary) -> Result<Vec<Elementary>, Error> {
    let mut terms: Vec<Elementary> = Vec::new();

    match polynomial {
        Add(pol1, pol2) => {
            for pol in get_terms((*pol1).clone())? {
                terms.push(pol);
            }
            for pol in get_terms((*pol2).clone())? {
                terms.push(pol);
            }
        }
        Sub(pol1, pol2) => {
            for pol in get_terms((*pol1).clone())? {
                terms.push(pol);
            }
            for pol in get_terms((*pol2).clone())? {
                terms.push(Mul(pol.into(), Con(-1.).into()));
            }
        }
        Mul(ref pol1, ref pol2) => {
            if pol1.is_constant() {
                for mut term in get_terms(simplify_polynomial((**pol2).clone())?)? {
                    term *= Con((**pol1).clone().call()(0.));
                    terms.push(term);
                }
            } else if pol2.is_constant() {
                for mut term in get_terms(simplify_polynomial((**pol1).clone())?)? {
                    term *= (**pol2).clone();
                    terms.push(term);
                }
            } else {
                terms.push(polynomial)
            }
        }
        Div(ref pol1, ref pol2) => {
            if pol2.is_constant() {
                for mut term in get_terms((**pol1).clone())? {
                    term /= (**pol2).clone();
                    terms.push(term);
                }
            } else {
                terms.push(polynomial)
            }
        }
        _ => terms.push(polynomial),
    }

    Ok(terms)
}

fn expand_term(polynomial: &Elementary) -> Result<Vec<Elementary>, Error> {
    match polynomial.clone() {
        Mul(pol1, pol2) => {
            let mut expansion: Vec<Elementary> = Vec::new();
            let terms1 = get_terms((*pol1).clone())?;
            let terms2 = get_terms((*pol2).clone())?;

            for term1 in terms1 {
                for term2 in terms2.clone() {
                    expansion.append(&mut expand_mul(&term1, &term2)?);
                }
            }

            Ok(expansion)
        }
        Pow(pol1, power) => {
            if let X = (*pol1).clone() {
                return Ok(vec![polynomial.to_owned()]);
            } else if let Con(_) = (*pol1).clone() {
                // this polynomial should be a constant
                Ok(vec![polynomial.clone().simplify()?])
            } else {
                // multinomal theorem
                if let Con(exp) = (*power).clone() {
                    let mut expansion: Vec<Elementary> = Vec::new();
                    for term in expand_multinomal(get_terms((*pol1).clone())?, exp as usize)? {
                        expansion.push(term.simplify_operations()?);
                    }
                    Ok(expansion)
                } else {
                    Err(Error::SimplifyError(polynomial.clone(), String::from("Attempted to perform polynomial simplification on a non-polynomial expression")))
                }
            }
        }
        _ => Ok(vec![polynomial.to_owned()]),
    }
}

// helper function for expand_term
fn expand_mul(term1: &Elementary, term2: &Elementary) -> Result<Vec<Elementary>, Error> {
    let mut expansion: Vec<Elementary> = Vec::new();

    let simplified_term1 = match term1 {
        Pow(_, _) => term1.simplify_operations()?,
        _ => term1.clone(),
    };
    let simplified_term2 = match term2 {
        Pow(_, _) => term1.simplify_operations()?,
        _ => term2.clone(),
    };

    expansion.push(simplified_term1 * simplified_term2);

    Ok(expansion)
}

// convert polynomial to the form f(x) = ax^b
fn convert_term(polynomial: Elementary) -> Result<Elementary, Error> {
    match polynomial.clone() {
        Mul(pol1, pol2) => {
            if let Con(numb) = (*pol1).clone() {
                if let Mul(coefficient, power) = convert_term((*pol2).clone())? {
                    let new_coefficient = coefficient * Con(numb);
                    return Ok(Mul(new_coefficient.into(), power));
                }
            } else if let Con(numb) = (*pol2).clone() {
                if let Mul(coefficient, power) = convert_term((*pol1).clone())? {
                    let new_coefficient = coefficient * Con(numb);
                    return Ok(Mul(new_coefficient.into(), power));
                }
            }

            if pol1.is_constant() {
                convert_mul(pol1, pol2, &polynomial)
            } else if pol2.is_constant() {
                convert_mul(pol2, pol1, &polynomial)
            } else {
                Err(Error::SimplifyError(
            polynomial.clone(),
            String::from(
                "Attempted to perform polynomial simplification on a non-polynomial expression",
            )))
            }
        }
        Div(pol1, pol2) => {
            if pol2.is_constant() {
                if let Mul(mut coefficient, power) = convert_term((*pol1).clone())? {
                    let new_coefficient = (*coefficient).clone() / (*pol2).clone().call()(0.);
                    Ok(Mul(new_coefficient.into(), power))
                } else {
                    Err(Error::SimplifyError(polynomial.clone(), String::from("Internal Error, polynomial simplification yielded an unexpected result")))
                }
            } else {
                Err(Error::SimplifyError(
                    polynomial.clone(),
                    String::from(
                        "Attempted to simplify a rational polynomial as a regular polynomial",
                    ),
                ))
            }
        }
        Pow(base, exp) => match (*base).clone() {
            X => Ok(Mul(Con(1.).into(), polynomial.clone().into())),
            Con(_) => Ok(Mul(
                polynomial.clone().simplify_constant()?.into(),
                Pow(X.into(), Con(0.).into()).into(),
            )),
            _ => Err(Error::SimplifyError(
                polynomial.clone(),
                String::from(
                    "Attempted to simplify a polynomial whose base is neither X nor a constant",
                ),
            )),
        },
        X => Ok(Mul(Con(1.).into(), Pow(X.into(), Con(1.).into()).into())),
        Con(numb) => Ok(Mul(Con(numb).into(), Pow(X.into(), Con(0.).into()).into()).into()),
        _ => Err(Error::SimplifyError(
            polynomial.clone(),
            String::from("Attempted to simplify a non-polynomial using polynomial-simplification"),
        )),
    }
}

// helper function for the convert_term function
fn convert_mul(
    pol1: Arc<Elementary>,
    pol2: Arc<Elementary>,
    polynomial: &Elementary,
) -> Result<Elementary, Error> {
    if let Pow(base, exp) = (*pol2).clone() {
        if let X = (*base).clone() {
            if exp.is_digit()? {
                return Ok(polynomial.clone());
            } else {
                return Err(Error::SimplifyError(
                    polynomial.clone(),
                    String::from("Non-digit exponent was found during polynomial simplification"),
                ));
            }
        } else {
            return Ok(Mul(
                polynomial.clone().simplify_constant()?.into(),
                Pow(X.into(), Con(0.).into()).into(),
            ));
        }
    } else if let X = (*pol2).clone() {
        return Ok(Mul(pol1.into(), Pow(X.into(), Con(1.).into()).into()));
    } else if (*pol2).clone().is_constant() {
        return Ok(Mul(
            polynomial.clone().simplify_constant()?.into(),
            Pow(X.into(), Con(0.).into()).into(),
        ));
    } else {
        return Err(Error::SimplifyError(
            polynomial.clone(),
            String::from(
                "Attempted to perform polynomial simplification on a non-polynomail expression",
            ),
        ));
    }
}

fn group_together(terms: Vec<Elementary>) -> Result<HashMap<i128, f64>, Error> {
    let mut map: HashMap<i128, f64> = HashMap::new();

    for term in terms {
        if let Mul(coefficient, power) = term {
            if let Pow(base, exp) = (*power).clone() {
                // insert into the map
                let degree = (*exp).clone().call()(0.) as i128;
                let coefficient = (*coefficient).clone().call()(0.);
                if map.contains_key(&degree) {
                    let mut existing_value = map.get_mut(&degree).expect("This should not fail");
                    // the unwrapping should not fail because the previous step ensures that the
                    // key does not already exist within the map
                    *existing_value += coefficient;
                } else {
                    map.insert(degree, coefficient);
                }
            }
        }
    }

    Ok(map)
}

fn polynomial_long_division(polynomial: &Elementary) -> Result<Elementary, Error> {
    unimplemented!()
}

// Multinomial expansion
fn expand_multinomal(terms: Vec<Elementary>, exponent: usize) -> Result<Vec<Elementary>, Error> {
    let n = &terms.len();
    let combinations = generate_combinations(*n);

    let mut res_terms: Vec<Elementary> = Vec::new();

    for i in 0..combinations.len() {
        let coefficient = Con(multinomial_coefficient(exponent, &combinations[i]) as f64);
        let mut new_term = coefficient;

        let terms = terms.clone();
        for j in 0..terms.len() {
            new_term *= Pow(
                terms[j].clone().into(),
                Con(combinations[i][j] as f64).into(),
            );
        }

        new_term.simplify()?;

        res_terms.push(new_term);
    }

    Ok(res_terms)
}

fn multinomial_coefficient(n_terms: usize, indexes: &Vec<usize>) -> usize {
    let mut denomenator = 1;
    for index in indexes {
        denomenator *= factorial(*index);
    }

    factorial(n_terms) / denomenator
}

fn generate_combinations(len: usize) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = Vec::new();
    let comb = vec![0; len];

    iterate_combination(comb, &mut combinations);

    combinations.clear_duplicates();

    combinations
}

fn iterate_combination(comb: Vec<usize>, combinations: &mut Vec<Vec<usize>>) {
    for i in 0..comb.len() {
        let mut new_comb = comb.clone();
        new_comb[i] += 1;

        let sum: usize = new_comb.iter().sum();
        if sum == comb.len() {
            combinations.push(new_comb);
        } else {
            iterate_combination(new_comb, combinations);
        }
    }
}

trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
    fn clear_duplicates(&mut self) {
        let mut already_seen = Vec::new();
        self.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        })
    }
}
