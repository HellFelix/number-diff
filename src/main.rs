use crate::Elementary::*;
use std::{
    f64::consts::{E, PI},
    sync::Arc,
};

fn main() {
    let enum_func = Log(Arc::new(X), Arc::new(Pow(Arc::new(Con(E)), Arc::new(X))));
    let derivative = enum_func.differentiate();
    let func = derivative.call();

    println!("{}", func(4.));
}

type Func = Box<dyn Fn(f64) -> f64 + 'static>;

#[derive(Debug, Clone)]
enum Elementary {
    Sin(Arc<Elementary>), // of the type sin(f(x))
    Cos(Arc<Elementary>), // of the type cos(f(x))
    Tan(Arc<Elementary>), // of the type tan(f(x))

    Add(Arc<Elementary>, Arc<Elementary>), // of the type f(x) + g(x)
    Sub(Arc<Elementary>, Arc<Elementary>), // of the type f(x) - g(x)
    Mul(Arc<Elementary>, Arc<Elementary>), // of the type f(x) * g(x)
    Div(Arc<Elementary>, Arc<Elementary>), // of the type f(x) / g(x)
    Pow(Arc<Elementary>, Arc<Elementary>), // of the type f(x)^g(x)
    Log(Arc<Elementary>, Arc<Elementary>), // of the type logb(f(x)) where b = g(x)

    Con(f64), // of the type c
    X,        // unit function f(x) = x
}
impl Elementary {
    pub fn call(self) -> Func {
        Box::new(move |x| match self.clone() {
            Sin(func) => (*func).clone().call()(x).sin(),
            Cos(func) => (*func).clone().call()(x).cos(),
            Tan(func) => (*func).clone().call()(x).tan(),

            Add(func1, func2) => (*func1).clone().call()(x) + (*func2).clone().call()(x),
            Sub(func1, func2) => (*func1).clone().call()(x) - (*func2).clone().call()(x),
            Mul(func1, func2) => (*func1).clone().call()(x) * (*func2).clone().call()(x),
            Div(func1, func2) => (*func1).clone().call()(x) / (*func2).clone().call()(x),

            Pow(func1, func2) => (*func1).clone().call()(x).powf((*func2).clone().call()(x)),
            Log(func1, func2) => (*func2).clone().call()(x).log((*func1).clone().call()(x)),

            Con(numb) => numb,
            X => f()(x),
        })
    }

    pub fn differentiate(self) -> Self {
        match self.clone() {
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
                    Arc::new(Pow(Arc::new(X), Arc::new(Con(2.)))),
                )),
                Arc::new((*func).clone().differentiate()),
            ), // 1/cos^2(f(x)) * f'(x)

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
            Pow(func1, func2) => Mul(Arc::new(Pow(func1.clone(), Arc::new(Sub(func2.clone(), Arc::new(Con(1.)))))), // f(x)^(g(x) - 1) 
                Arc::new(Add(Arc::new(Mul(func2.clone(), Arc::new((*func1).clone().differentiate()))),  // g(x)f'(x)
                    Arc::new(Mul(func1.clone(), // f(x)
                    Arc::new(Mul(Arc::new(Log(Arc::new(Con(E)), func1.clone())), // ln(f(x))
                    Arc::new((*func2).clone().differentiate())))))))), // g'(x)
            // f(x)^(g(x) - 1) (g(x) f'(x) + f(x) log(f(x)) g'(x))
            Log(func1, func2) => Div(Arc::new(Sub(
                Arc::new(Div(Arc::new(Mul(Arc::new(Log(Arc::new(Con(E)), func1.clone())), Arc::new((*func2).clone().differentiate()))), func2.clone())), 
                Arc::new(Div(Arc::new(Mul(Arc::new(Log(Arc::new(Con(E)), func2.clone())), Arc::new((*func1).clone().differentiate()))), func1.clone())))),
                Arc::new(Pow(Arc::new(Log(Arc::new(Con(E)), func1.clone())), Arc::new(Con(2.))))),
            Con(_) => Con(0.),
            X => Con(1.),
        }
    }
}

// unit function f(x) -> x
fn f() -> Func {
    Box::new(move |x| x)
}
