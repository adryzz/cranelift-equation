mod ast;

use crate::ast::FunctionType;
use ast::{Operator, ParenthesisType, RawSyntax, Syntax};
use std::str::FromStr;
use thiserror::Error;

pub fn parse<T: num_traits::Float + std::fmt::Debug + std::fmt::Display>(equation: &str) {
    // first pass
    let first = first_parse(equation).unwrap();
    //dbg!(&first);
    let second = second_parse::<T>(&first[..], equation).unwrap();
    dbg!(&second);

    print!("{} => ", equation);
    print::<T>(&second[..]);
}

fn first_parse(equation: &str) -> Result<Vec<RawSyntax>, EquationParseError> {
    let mut vec = Vec::new();

    let mut last_start_index: Option<(usize, bool)> = None;

    for (index, value) in equation.chars().enumerate() {
        if value.is_numeric() || value == '.' {
            match last_start_index {
                None => {}
                Some((_, false)) => continue,
                Some((start, true)) => {
                    vec.push(RawSyntax::ValueIdent { start, end: index });
                    vec.push(RawSyntax::Operator(Operator::Multiply));
                }
            }

            last_start_index = Some((index, false));
            continue;
        } else if value.is_alphabetic() {
            match last_start_index {
                None => {}
                Some((_, true)) => continue,
                Some((start, false)) => {
                    vec.push(RawSyntax::ValueLit { start, end: index });
                    vec.push(RawSyntax::Operator(Operator::Multiply));
                }
            }
            last_start_index = Some((index, true));
            continue;
        } else {
            match last_start_index {
                None => {}
                Some((start, false)) => {
                    vec.push(RawSyntax::ValueLit { start, end: index });
                }
                Some((start, true)) => {
                    if value == '(' {
                        vec.push(RawSyntax::Function { start, end: index });
                    } else {
                        vec.push(RawSyntax::ValueIdent { start, end: index });
                        vec.push(RawSyntax::Operator(Operator::Multiply));
                    }
                }
            }
            last_start_index = None;
        }

        match value {
            ' ' => {
                continue;
            }
            ',' => {
                vec.push(RawSyntax::Comma);
                continue;
            }
            _ => {}
        }

        if let Ok(val) = ParenthesisType::try_from(value) {
            vec.push(RawSyntax::Parenthesis(val));
            continue;
        }

        if let Ok(val) = Operator::try_from(value) {
            vec.push(RawSyntax::Operator(val));
            continue;
        }
    }

    match last_start_index {
        None => {}
        Some((start, false)) => {
            vec.push(RawSyntax::ValueLit { start, end: equation.len() });
        }
        Some((start, true)) => {
            vec.push(RawSyntax::ValueIdent { start, end: equation.len() });
        }
    }

    Ok(vec)
}

fn second_parse<'a, T: num_traits::Float + std::fmt::Debug>(
    ast: &[RawSyntax],
    equation: &'a str,
) -> Result<Vec<Syntax<'a, T>>, EquationParseError> {
    let mut vec = Vec::with_capacity(ast.len());
    for token in ast {
        match token {
            RawSyntax::ValueLit { start, end } => {
                match T::from_str_radix(&equation[*start..*end], 10) {
                    Ok(v) => vec.push(Syntax::ValueLit(v)),
                    Err(_) => return Err(EquationParseError::LiteralParseError),
                }
            }
            RawSyntax::ValueIdent { start, end } => {
                vec.push(Syntax::ValueIdent(&equation[*start..*end]));
            }
            RawSyntax::Operator(operator) => vec.push(Syntax::Operator(*operator)),
            RawSyntax::Parenthesis(parenthesis_type) => {
                vec.push(Syntax::Parenthesis(*parenthesis_type));
            }
            RawSyntax::Function { start, end } => vec.push(Syntax::Function(
                FunctionType::from_str(&equation[*start..*end])?,
            )),
            RawSyntax::Comma => vec.push(Syntax::Comma),
        }
    }

    Ok(vec)
}

fn print<'a, T: num_traits::Float + std::fmt::Debug + std::fmt::Display>(ast: &[Syntax<'a, T>]) {
    for token in ast {
        match token {
            Syntax::ValueLit(val) => print!("{} ", val),
            Syntax::ValueIdent(ident) => print!("{} ", ident),
            Syntax::Operator(op) => match op {
                Operator::Add => print!("+ "),
                Operator::Subtract => print!("- "),
                Operator::Multiply => print!("* "),
                Operator::Divide => print!("/ "),
            },
            Syntax::Parenthesis(p) => match p {
                ParenthesisType::Open => print!("( "),
                ParenthesisType::Close => print!(") "),
                ParenthesisType::OpenSquare => print!("[ "),
                ParenthesisType::CloseSquare => print!("] "),
                ParenthesisType::OpenCurly => print!("{{ "),
                ParenthesisType::CloseCurly => print!("}} "),
            },
            Syntax::Comma => print!(", "),
            Syntax::Function(func) => print!("{:?}", func),
        }
    }
    println!("=");
}

#[derive(Debug, Error)]
pub enum EquationParseError {
    #[error("Bad literal")]
    LiteralParseError,
    #[error("Unknown function")]
    UnknownFunction,
    #[error("No character match")]
    NoMatch,
}
