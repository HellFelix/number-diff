use crate::{
    Elementary::{self, *},
    Error, Function,
};
use std::{f64::consts::E, sync::Arc};
impl Elementary {
    fn differentiate(self) -> Self {
        match self {
            Sin(func) => Mul(
                Arc::new(Cos(func.clone())),
                Arc::new((*func).clone().differentiate()),
            ), // cos(f(x))*f'(x)
            Cos(func) => Mul(
                Arc::new(Mul(Arc::new(Sin(func.clone())), Arc::new(Con(-1.)))),
                Arc::new((*func).clone().differentiate()),
            ), // -sin(f(x))*f'(x)
            Tan(func) => Mul(
                Arc::new(Div(
                    Arc::new(Con(1.)),
                    Arc::new(Pow(Arc::new(Cos(Arc::new(X))), Arc::new(Con(2.)))),
                )),
                Arc::new((*func).clone().differentiate()),
            ), // 1/cos^2(f(x)) * f'(x)

            Sec(func) => (*func).clone().differentiate() * Tan(func.clone()) * Sec(func.to_owned()),
            Csc(func) => {
                (*func).clone().differentiate()
                    * Cot(func.clone())
                    * Csc(func.to_owned())
                    * (-1 as f64)
            }
            Cot(func) => {
                (*func).clone().differentiate()
                    * (-1 as f64)
                    * Pow(Arc::new(Csc(func.to_owned())), Arc::new(Con(2.)))
            }

            Asin(func) => Div(
                Arc::new((*func).clone().differentiate()),
                Arc::new(Pow(
                    Arc::new(Sub(
                        Arc::new(Con(1.)),
                        Arc::new(Pow(func.clone(), Arc::new(Con(2.)))),
                    )),
                    Arc::new(Con(0.5)),
                )),
            ),
            Acos(func) => Mul(
                Arc::new(Div(
                    Arc::new((*func).clone().differentiate()),
                    Arc::new(Pow(
                        Arc::new(Sub(
                            Arc::new(Con(1.)),
                            Arc::new(Pow(func.clone(), Arc::new(Con(2.)))),
                        )),
                        Arc::new(Con(0.5)),
                    )),
                )),
                Arc::new(Con(-1.)),
            ),
            Atan(func) => Div(
                Arc::new((*func).clone().differentiate()),
                Arc::new(Add(
                    Arc::new(Pow(func.clone(), Arc::new(Con(2.)))),
                    Arc::new(Con(1.)),
                )),
            ),
            Sinh(func) => Mul(
                Arc::new(Cosh(func.clone())),
                Arc::new((*func).clone().differentiate()),
            ),
            Cosh(func) => Mul(
                Arc::new(Sinh(func.clone())),
                Arc::new((*func).clone().differentiate()),
            ),
            Tanh(func) => Div(
                Arc::new((*func).clone().differentiate()),
                Arc::new(Pow(Arc::new(Cosh(func.clone())), Arc::new(Con(2.)))),
            ),

            Add(func1, func2) => Add(
                Arc::new((*func1).clone().differentiate()),
                Arc::new((*func2).clone().differentiate()),
            ), // f'(x) + g'(x)
            Sub(func1, func2) => Sub(
                Arc::new((*func1).clone().differentiate()),
                Arc::new((*func2).clone().differentiate()),
            ), // f'(x) - g'(x)
            Mul(func1, func2) => Add(
                Arc::new(Mul(
                    Arc::new((*func1).clone().differentiate()),
                    func2.clone(),
                )),
                Arc::new(Mul(
                    Arc::new((*func2).clone().differentiate()),
                    func1.clone(),
                )),
            ), //f'(x)*g(x) + f(x)*g'(x)
            Div(func1, func2) => Div(
                Arc::new(Sub(
                    Arc::new(Mul(
                        Arc::new((*func1).clone().differentiate()),
                        func2.clone(),
                    )),
                    Arc::new(Mul(
                        Arc::new((*func2).clone().differentiate()),
                        func1.clone(),
                    )),
                )),
                Arc::new(Pow((func2).clone(), Arc::new(Con(2.)))),
            ), // (f'(x)g(x) - f(x)g'(x)) / (g(x))^2
            Pow(func1, func2) => Mul(
                Arc::new(Pow(
                    func1.clone(),
                    Arc::new(Sub(func2.clone(), Arc::new(Con(1.)))),
                )), // f(x)^(g(x) - 1)
                Arc::new(Add(
                    Arc::new(Mul(
                        func2.clone(),
                        Arc::new((*func1).clone().differentiate()),
                    )), // g(x)f'(x)
                    Arc::new(Mul(
                        func1.clone(), // f(x)
                        Arc::new(Mul(
                            Arc::new(Log(Arc::new(Con(E)), func1.clone())), // ln(f(x))
                            Arc::new((*func2).clone().differentiate()),
                        )),
                    )),
                )),
            ), // g'(x)
            // f(x)^(g(x) - 1) (g(x) f'(x) + f(x) log(f(x)) g'(x))
            Log(func1, func2) => Div(
                Arc::new(Sub(
                    Arc::new(Div(
                        Arc::new(Mul(
                            Arc::new(Log(Arc::new(Con(E)), func1.clone())),
                            Arc::new((*func2).clone().differentiate()),
                        )),
                        func2.clone(),
                    )),
                    Arc::new(Div(
                        Arc::new(Mul(
                            Arc::new(Log(Arc::new(Con(E)), func2.clone())),
                            Arc::new((*func1).clone().differentiate()),
                        )),
                        func1.clone(),
                    )),
                )),
                Arc::new(Pow(
                    Arc::new(Log(Arc::new(Con(E)), func1.clone())),
                    Arc::new(Con(2.)),
                )),
            ),

            // f'(x) * f(x)! * 𝝍0(f(x)+1)
            Factorial(func) => {
                (*func).clone().differentiate()
                    * Factorial(func.clone())
                    * Polygamma((func + Con(1.)).into(), 0)
            }

            // 𝜞'(f(x)) = f'(x)*𝜞(f(x))*𝝍0(f(x))
            Gamma(func) => {
                (*func).clone().differentiate() * Gamma(func.clone()) * Polygamma(func, 0)
            }
            // 𝝍(m)'(f(x)) = f'(x)*𝝍(m+1)(f(x))
            Polygamma(func, order) => (*func).clone().differentiate() * Polygamma(func, order + 1),

            Abs(func) => Div(
                Arc::new(Mul(func.clone(), Arc::new((*func).clone().differentiate()))),
                Arc::new(Abs(func)),
            ),
            Con(_) => Con(0.),
            X => Con(1.),
        }
    }
    pub fn derivative(self) -> Result<Elementary, Error> {
        self.differentiate().simplify()
    }
    pub fn derivative_unsimplified(self) -> Elementary {
        self.differentiate()
    }
}

/// Returns the derivative of the passed [Function](crate::Function).
///
/// Example:
/// ```rust
/// let sin = Function::from("sin(x)");
///
/// let sin_derivative = derivative_of(&sin);
/// // sin(x)' = cos(x)
/// // cos(0) = 1
/// assert_eq!(sin_derivative.call(0.), 1.)
/// ```
pub fn derivative_of(input_func: &Function) -> Function {
    let elem_derivative = input_func.elementary().clone().differentiate();
    Function::from(elem_derivative)
}
