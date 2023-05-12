use std::sync::Arc;

use crate::Elementary::{self, *};

impl Elementary {
    pub fn test_from<'a>(value: &'a String) -> Self {
        Self::to_elementary(value)
    }

    fn split_function<'a>(value: &'a String) -> Vec<&'a str> {
        let mut interp_slice: Vec<&str> = value.split("").collect();
        // remove the first and last element because they are just empty string slices
        interp_slice.remove(0);
        interp_slice.pop();

        let mut chunks: Vec<&str> = Vec::new();
        let mut open_parenthesis = -1;

        let mut cut_index = 0;
        for i in 0..interp_slice.len() {
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

            // detect operations
            if open_parenthesis == -1 {
                if interp_slice[i] == "+"
                    || interp_slice[i] == "-"
                    || interp_slice[i] == "*"
                    || interp_slice[i] == "/"
                    || interp_slice[i] == "^"
                {
                    chunks.push(interp_slice[i]);
                    cut_index = i + 1;
                } else if interp_slice[i] == "x" {
                    chunks.push(&value[cut_index..=i]);
                    cut_index = i + 1;
                }
            }
        }

        if chunks.is_empty() {
            chunks.push(&value[..]);
        }

        chunks
    }

    fn to_elementary<'a>(string: &String) -> Self {
        let strings = Self::split_function(string);

        let mut functions: Vec<ElemRef> = strings
            .clone()
            .iter()
            .map(|s| {
                let val = Self::parse_function(s).unwrap();
                val
            })
            .collect();

        // order of operations
        // note that the order of operations have to go backwards
        while functions.len() != 1 {
            // first in the order of operations is powers (seeing as parentheses are handled as a
            // separate case)
            if functions.contains(&ElemRef::Pow) {
                for i in (0..functions.len()).rev() {
                    // find the index of the last power (because we treat this case from right to
                    // left)
                    if i >= functions.len() {
                        continue;
                    }
                    match &functions[i] {
                        ElemRef::Pow => {
                            let replacement_func = ElemRef::Function(Pow(
                                Arc::new(functions[i - 1].clone().convert().unwrap()),
                                Arc::new(functions[i + 1].clone().convert().unwrap()),
                            ));
                            functions.remove(i + 1);
                            functions.remove(i);
                            functions.remove(i - 1);
                            functions.insert(i - 1, replacement_func);
                        }
                        _ => (),
                    }
                }

                continue;
            }

            // next up in the order of operations is multiplication
            while functions.contains(&ElemRef::Mul) {
                iterate_operation(&mut functions, ElemRef::Mul);
            }

            // next up is division
            while functions.contains(&ElemRef::Div) {
                iterate_operation(&mut functions, ElemRef::Div);
            }

            // then addition
            while functions.contains(&ElemRef::Add) {
                iterate_operation(&mut functions, ElemRef::Add);
            }

            // and lastly subtracion
            while functions.contains(&ElemRef::Sub) {
                iterate_operation(&mut functions, ElemRef::Sub);
            }
        }

        functions.pop().expect("empty").convert().unwrap()
    }

    fn parse_function(string: &str) -> Option<ElemRef> {
        let mut string = string.to_lowercase();

        // unwrap potential parentheses
        if &string[..1] == "(" {
            while &string[..1] == "(" {
                string = string[1..string.len() - 1].to_string();
            }
            return Some(ElemRef::Function(Self::to_elementary(&string)));
        }

        if string == "x" {
            return Some(ElemRef::Function(X));
        }

        match &string[..] {
            // check in order of operations
            "^" => Some(ElemRef::Pow),
            "*" => Some(ElemRef::Mul),
            "/" => Some(ElemRef::Div),
            "+" => Some(ElemRef::Add),
            "-" => Some(ElemRef::Sub),
            &_ => {
                // if we do not have an operation, we must have a function consisting of a function
                // identifier and its contents
                let (func, cont) = split_first(&string, "(");

                // remove parenthesis
                let cont = cont[1..cont.len() - 1].to_string();

                match func {
                    "sin" => Some(ElemRef::Function(Sin(Arc::new(Self::to_elementary(
                        &cont.to_string(),
                    ))))),
                    "cos" => Some(ElemRef::Function(Cos(Arc::new(Self::to_elementary(
                        &cont.to_string(),
                    ))))),
                    _ => None,
                }
            }
        }
    }
}

fn iterate_operation(functions: &mut Vec<ElemRef>, operation: ElemRef) {
    if functions.contains(&operation) {
        for i in 0..functions.len() {
            if i >= functions.len() {
                continue;
            }

            if &functions[i] == &operation {
                let replacement_func = match operation {
                    ElemRef::Mul => ElemRef::Function(Mul(
                        Arc::new(functions[i - 1].clone().convert().unwrap()),
                        Arc::new(functions[i + 1].clone().convert().unwrap()),
                    )),
                    ElemRef::Div => ElemRef::Function(Div(
                        Arc::new(functions[i - 1].clone().convert().unwrap()),
                        Arc::new(functions[i + 1].clone().convert().unwrap()),
                    )),
                    ElemRef::Add => ElemRef::Function(Add(
                        Arc::new(functions[i - 1].clone().convert().unwrap()),
                        Arc::new(functions[i + 1].clone().convert().unwrap()),
                    )),
                    ElemRef::Sub => ElemRef::Function(Sub(
                        Arc::new(functions[i - 1].clone().convert().unwrap()),
                        Arc::new(functions[i + 1].clone().convert().unwrap()),
                    )),
                    _ => unimplemented!(),
                };
                functions.remove(i + 1);
                functions.remove(i);
                functions.remove(i - 1);
                functions.insert(i - 1, replacement_func);
            }
        }
    }
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
}
impl ElemRef {
    fn convert(self) -> Option<Elementary> {
        match self {
            Self::Function(elem) => Some(elem),
            _ => None,
        }
    }
}

fn split_first<'a>(string: &'a String, indentifier: &'a str) -> (&'a str, &'a str) {
    // find index of first insance of the identifier
    let slice: Vec<&str> = string.split("").collect();

    let mut index = 0;
    for i in 0..string.len() {
        if slice[i] == indentifier {
            index = i;
        }
    }

    string.split_at(index - 1)
}
