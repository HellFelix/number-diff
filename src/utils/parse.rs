use crate::Error;
use std::{
    f64::consts::{E, PI},
    sync::Arc,
};

use crate::Elementary::{self, *};

impl<'a> From<&'a str> for Elementary {
    fn from(value: &'a str) -> Self {
        let value: String = value.split_whitespace().collect();
        Self::to_elementary(&value).unwrap()
    }
}
impl Elementary {
    fn split_function(value: &str) -> Vec<&str> {
        let mut interp_slice: Vec<&str> = value.split("").collect();
        // remove the first and last element because they are just empty string slices
        interp_slice.remove(0);
        interp_slice.pop();

        let mut chunks: Vec<&str> = Vec::new();
        let mut open_parenthesis = -1;

        let mut cut_index = 0;

        let mut skip = 0;
        for i in 0..interp_slice.len() {
            // if items need to be skipped (because of the implementation of constants)
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if interp_slice[i] == "(" {
                // this is for the first case of an opening parenthesis. Note that we cannot start
                // at 0 since that would match the case for closing an outer parenthesis
                if open_parenthesis == -1 {
                    open_parenthesis = 1;
                } else {
                    // for all other cases, however, the number of open parentheses just goes up by
                    // one
                    open_parenthesis += 1;
                }
            } else if interp_slice[i] == ")" {
                open_parenthesis -= 1
            }

            // check if outer parenthesis has been closed
            if open_parenthesis == 0 {
                chunks.push(&value[cut_index..=i]);

                // set new cut index
                cut_index = i + 1;

                // reset parenthesis
                open_parenthesis = -1;
            }

            // detect operations and constants
            if open_parenthesis == -1 {
                if interp_slice[i] == "+"
                    || interp_slice[i] == "-"
                    || interp_slice[i] == "*"
                    || interp_slice[i] == "/"
                    || interp_slice[i] == "^"
                {
                    chunks.push(interp_slice[i]);
                    cut_index = i + 1;
                } else if interp_slice[i] == "!" {
                    chunks.push(&value[cut_index..=i]);
                    cut_index = i + 1;
                } else if interp_slice[i] == "x" {
                    chunks.push(&value[cut_index..=i]);
                    cut_index = i + 1;
                } else if interp_slice[i] == "e" {
                    chunks.push(&value[cut_index..=i]);
                    cut_index = i + 1;
                } else if interp_slice[i] == "pi" || interp_slice[i] == "π" {
                    chunks.push("pi");
                    cut_index = i + 1;
                } else {
                    // checking for numbers
                    if let Ok(_) = &value[cut_index..=i].parse::<f64>() {
                        // find the index at which the number ends
                        let mut last_index = i;
                        'index: for j in i + 1..=interp_slice.len() {
                            if let Ok(_) = &value[cut_index..j].parse::<f64>() {
                                last_index = j - 1;
                            } else {
                                break 'index;
                            }
                        }

                        // push the whole number
                        chunks.push(&value[cut_index..=last_index]);

                        // the next couple of indexes must be skipped in order to avoid parsing of
                        // individual digits
                        skip = last_index - i;
                        // by setting skip to the number of difference between the current index
                        // and the index at which the number ends
                        cut_index = last_index + 1;
                    }
                }
            }
        }

        if chunks.is_empty() {
            chunks.push(value);
        }

        chunks
    }

    fn to_elementary(string: &str) -> Result<Self, Error> {
        let strings = Self::split_function(string);

        let mut functions: Vec<ElemRef> = strings
            .clone()
            .iter()
            .map(|s| Self::parse_function(s).unwrap())
            .collect();

        let mut iteration = 0;

        // order of operations
        while functions.len() != 1 {
            if iteration >= 10000 {
                return Err(Error::ParseError(String::from(format!(
                    "Iteration limit reached while parsing function: {string}",
                ))));
            } else {
                iteration += 1;
            }
            // first in the order of operations is powers (seeing as parentheses are handled as a
            // separate case)
            if functions.contains(&ElemRef::Pow) {
                for i in (0..functions.len()).rev() {
                    // find the index of the last power (because we treat this case from right to
                    // left)
                    if i >= functions.len() {
                        continue;
                    }
                    if functions[i] == ElemRef::Pow {
                        let replacement_func = ElemRef::Function(Pow(
                            Arc::new(functions[i - 1].clone().convert()?),
                            Arc::new(functions[i + 1].clone().convert()?),
                        ));
                        functions.remove(i + 1);
                        functions.remove(i);
                        functions.remove(i - 1);
                        functions.insert(i - 1, replacement_func);
                    }
                }

                continue;
            }

            // the factorial function (x ↦ x!) is by convention written in "postfix" notation (i.e.
            // it taeks precedence over normal operations)
            if functions.contains(&ElemRef::Factorial) {
                iterate_operation(&mut functions, ElemRef::Factorial)?;
                continue;
            }

            // next up in the order of operations is multiplication
            if functions.contains(&ElemRef::Mul) {
                iterate_operation(&mut functions, ElemRef::Mul)?;
                continue;
            }

            // we also have to handle implied multiplication. Weather this is handled before or
            // after the explicit multiplication doesn't matter since multiplication is commutative
            // i.e. a*b = b*a

            // check if there is there are any instances of implied multiplication
            for i in 0..functions.len() {
                if i < functions.len() - 1 {
                    if let (ElemRef::Function(func1), ElemRef::Function(func2)) =
                        (&functions[i], &functions[i + 1])
                    {
                        // multiply the two together
                        let replacement_func = ElemRef::Function(Mul(
                            Arc::new(func1.to_owned()),
                            Arc::new(func2.to_owned()),
                        ));

                        // remove the functions and replace them with the multiplied function
                        functions.remove(i + 1);
                        functions.remove(i);
                        functions.insert(i, replacement_func);
                    }
                }
            }
            // next up is division
            if functions.contains(&ElemRef::Div) {
                iterate_operation(&mut functions, ElemRef::Div)?;
                continue;
            }

            // then addition
            if functions.contains(&ElemRef::Add) {
                iterate_operation(&mut functions, ElemRef::Add)?;
                continue;
            }

            // and lastly subtracion
            if functions.contains(&ElemRef::Sub) {
                iterate_operation(&mut functions, ElemRef::Sub)?;
                continue;
            }
        }

        functions
            .pop()
            .expect("Couldn't find a function to parse")
            .convert()
    }

    fn parse_function(string: &str) -> Result<ElemRef, Error> {
        let mut string = string.to_lowercase();

        // unwrap potential parentheses
        if &string[..1] == "(" {
            while &string[..1] == "(" {
                string = string[1..string.len() - 1].to_string();
            }
            return Ok(ElemRef::Function(Self::to_elementary(&string)?));
        }

        // check for special function (independent variable) x, and then check for constants
        if string == "x" {
            return Ok(ElemRef::Function(X));
        } else if let Ok(number) = string.parse::<f64>() {
            return Ok(ElemRef::Function(Con(number)));
        }

        match &string[..] {
            // check in order of operations
            "^" => Ok(ElemRef::Pow),
            "*" => Ok(ElemRef::Mul),
            "/" => Ok(ElemRef::Div),
            "+" => Ok(ElemRef::Add),
            "-" => Ok(ElemRef::Sub),
            // also constants
            "e" => Ok(ElemRef::Function(Con(E))),
            "pi" => Ok(ElemRef::Function(Con(PI))),
            "!" => Ok(ElemRef::Factorial),
            _ => {
                // if we do not have an operation, we must have a function consisting of a function
                // identifier and its contents
                let (func, cont) = split_first(&string, "(");

                // remove outer parenthesis
                let mut graphemes: Vec<&str> = cont.split("").collect();
                // remove the first and last character because they should be parentheses
                graphemes.remove(0);
                graphemes.pop();
                let cont: String = graphemes.iter().map(|x| *x).collect();

                match func {
                    "sin" => Ok(ElemRef::Function(Sin(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "cos" => Ok(ElemRef::Function(Cos(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "tan" => Ok(ElemRef::Function(Tan(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "sec" => Ok(ElemRef::Function(Sec(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "csc" => Ok(ElemRef::Function(Csc(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "cot" => Ok(ElemRef::Function(Cot(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "asin" => Ok(ElemRef::Function(Asin(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "acos" => Ok(ElemRef::Function(Acos(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "atan" => Ok(ElemRef::Function(Atan(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "sinh" => Ok(ElemRef::Function(Sinh(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "cosh" => Ok(ElemRef::Function(Cosh(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "tanh" => Ok(ElemRef::Function(Tanh(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "ln" => Ok(ElemRef::Function(Log(
                        Arc::new(Con(E)), //ln is equivalent to log base e of its contents
                        Arc::new(Self::to_elementary(&cont)?),
                    ))),
                    "abs" => Ok(ElemRef::Function(Abs(Arc::new(Self::to_elementary(
                        &cont,
                    )?)))),
                    "sqrt" => Ok(ElemRef::Function(Pow(
                        Arc::new(Self::to_elementary(&cont)?),
                        Arc::new(Con(0.5)),
                    ))),
                    "d" => Ok(ElemRef::Function(
                        Self::to_elementary(&cont)?.derivative_unsimplified(),
                    )),
                    _ => Err(Error::ParseError(format!(
                        "Function identifier '{func}' not recognized"
                    ))),
                }
            }
        }
    }
}

// all instances of an operation must be handled before the parsing method can move on to the next.
// This is to ensure that the order of operations is being upheld
fn iterate_operation(functions: &mut Vec<ElemRef>, operation: ElemRef) -> Result<(), Error> {
    if functions.contains(&operation) {
        for i in 0..functions.len() {
            if i >= functions.len() {
                continue;
            }

            if functions[i] == operation {
                let replacement_func = match operation {
                    ElemRef::Mul => ElemRef::Function(Mul(
                        Arc::new(functions[i - 1].clone().convert()?),
                        Arc::new(functions[i + 1].clone().convert()?),
                    )),
                    ElemRef::Div => ElemRef::Function(Div(
                        Arc::new(functions[i - 1].clone().convert()?),
                        Arc::new(functions[i + 1].clone().convert()?),
                    )),
                    ElemRef::Add => ElemRef::Function(Add(
                        Arc::new(functions[i + 1].clone().convert()?),
                        Arc::new(functions[i - 1].clone().convert()?),
                    )),
                    ElemRef::Sub => {
                        if i > 0 {
                            ElemRef::Function(Sub(
                                Arc::new(functions[i - 1].clone().convert()?),
                                Arc::new(functions[i + 1].clone().convert()?),
                            ))
                        } else {
                            ElemRef::Function(functions[i + 1].clone().convert()? * (-1 as f64))
                        }
                    }
                    ElemRef::Factorial => {
                        if i > 0 {
                            ElemRef::Function(Factorial(functions[i - 1].clone().convert()?.into()))
                        } else {
                            return Err(Error::ParseError(String::from(
                                "Factorial function must be applied to another function",
                            )));
                        }
                    }
                    _ => unimplemented!("No such operation"), // this point shouldn't be reached
                };

                if operation == ElemRef::Factorial {
                    println!("testing - {replacement_func:?}");
                    // the factorial notation is rather unique, because it always follows directly
                    // after the function upon which it acts.
                    functions.remove(i);
                    functions.remove(i - 1);
                    functions.insert(i - 1, replacement_func)
                } else {
                    // the operation itself as well as the functions surrounding it must be removed
                    functions.remove(i + 1);
                    functions.remove(i);
                    if i > 0 {
                        functions.remove(i - 1);
                        // the combined new function is inserted in the place of the previous functions
                        functions.insert(i - 1, replacement_func);
                    } else {
                        // this is strictly for when a negative number is implied as seen above
                        functions.insert(i, replacement_func)
                    }
                }
            }
        }
    }
    Ok(())
}

// enum to allow operations to be described as the same type without carrying two functions
#[derive(Debug, Clone, PartialEq)]
enum ElemRef {
    Function(Elementary),
    Pow,
    Mul,
    Div,
    Add,
    Sub,
    Factorial,
}
impl ElemRef {
    fn convert(self) -> Result<Elementary, Error> {
        match self {
            Self::Function(elem) => Ok(elem),
            _ => Err(Error::ParseError(String::from(
                "Cannot convert operation to elementary function",
            ))),
        }
    }
}

// splits the provided string at the first index where the specified identifier is found.
// if the identifier is not found, the string will be split at index 0
fn split_first<'a>(string: &'a String, indentifier: &'a str) -> (&'a str, &'a str) {
    let slice: Vec<&str> = string.split("").collect();

    let mut index = 0;
    // find index of first insance of the identifier

    for (i, s) in slice.iter().enumerate().take(string.len()) {
        if *s == indentifier {
            index = i;
            break;
        }
    }

    string.split_at(index - 1)
}
