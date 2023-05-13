use crate::Elementary::{self, *};

#[derive(Debug)]
pub enum Category {
    Exponential,
    Polynomial,
    Trigonometric,
    Constant,
    ClusterFuck,
}

enum FuncRep {
    Sin,
    Cos,
    Tan,

    Asin,
    Acos,
    Atan,

    Sinh,
    Cosh,
    Tanh,

    Mul(Elementary),
    Div(Elementary),
    Pow(Elementary), // Pow must include an Elementary instance because the base and exponent must
    // both be identifiable.
    Log(Elementary),

    Con,
    Abs,
}

impl Elementary {
    pub fn classify(&self) -> Category {
        if self.is_constant() {
            Category::Constant
        } else {
            Category::ClusterFuck
        }
    }

    // checks if the provided function is constant, that is, it contains no independent variable
    // of the type f(x) = C
    fn is_constant(&self) -> bool {
        match self {
            Sin(func) => func.is_constant(),
            Cos(func) => func.is_constant(),
            Tan(func) => func.is_constant(),

            Asin(func) => func.is_constant(),
            Acos(func) => func.is_constant(),
            Atan(func) => func.is_constant(),

            Sinh(func) => func.is_constant(),
            Cosh(func) => func.is_constant(),
            Tanh(func) => func.is_constant(),

            // for the operations, both of their functions must be constant for the whole function
            // to be considered constant
            Add(func1, func2) => func1.is_constant() && func2.is_constant(),
            Sub(func1, func2) => func1.is_constant() && func2.is_constant(),
            Mul(func1, func2) => func1.is_constant() && func2.is_constant(),
            Div(func1, func2) => func1.is_constant() && func2.is_constant(),
            Pow(func1, func2) => func1.is_constant() && func2.is_constant(),
            Log(func1, func2) => func1.is_constant() && func2.is_constant(),

            Abs(func) => func.is_constant(),

            Con(_) => true,
            X => false,
        }
    }

    fn component_functions(&self) -> Vec<FuncRep> {
        let mut components: Vec<FuncRep> = Vec::new();

        match self {
            Sin(_) => {
                components.push(FuncRep::Sin);
            }
            Cos(_) => {
                components.push(FuncRep::Cos);
            }
            Tan(_) => {
                components.push(FuncRep::Tan);
            }

            Asin(_) => {
                components.push(FuncRep::Asin);
            }
            Acos(_) => {
                components.push(FuncRep::Acos);
            }
            Atan(_) => {
                components.push(FuncRep::Atan);
            }

            Sinh(_) => {
                components.push(FuncRep::Sinh);
            }
            Cosh(_) => {
                components.push(FuncRep::Cosh);
            }
            Tanh(_) => {
                components.push(FuncRep::Tanh);
            }

            Add(func1, func2) => {
                for c in func1.component_functions() {
                    components.push(c);
                }
                for c in func2.component_functions() {
                    components.push(c);
                }
            }
            Sub(func1, func2) => {
                for c in func1.component_functions() {
                    components.push(c);
                }
                for c in func2.component_functions() {
                    components.push(c);
                }
            }
            Mul(func1, func2) => {
                components.push(FuncRep::Mul(Mul(func1.clone(), func2.clone())));
            }
            Div(func1, func2) => {
                components.push(FuncRep::Div(Div(func1.clone(), func2.clone())));
            }
            Pow(func1, func2) => {
                components.push(FuncRep::Pow(Pow(func1.clone(), func2.clone())));
            }

            Abs(_) => {
                components.push(FuncRep::Abs);
            }

            Con(_) => {
                components.push(FuncRep::Con);
            }

            Log(func1, func2) => {
                components.push(FuncRep::Log(Log(func1.to_owned(), func2.to_owned())));
            }

            X => (),
        }

        components
    }
}
