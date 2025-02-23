mod ast;

use ast::{Operator, ParenthesisType, RawSyntax, Syntax};
use thiserror::Error;

pub fn parse<T>(equation: &str) {
    // first pass
    let first = first_parse(equation).unwrap();
    dbg!(&first);
}

fn first_parse(equation: &str) -> Result<Vec<RawSyntax>, EquationParseError> {
    let mut vec = Vec::new();

    let mut last_start_index: Option<usize> = None;

    for (index, value) in equation.chars().enumerate() {
        if value.is_alphanumeric() || value == '.' {
            if last_start_index.is_none() {
                last_start_index = Some(index);
            }
            continue;
        }

        match value {
            ' ' => {
                if let Some(start) = last_start_index {
                    vec.push(RawSyntax::Value { start, end: index });
                    last_start_index = None;
                }
                continue;
            },
            ',' => {
                if let Some(start) = last_start_index {
                    vec.push(RawSyntax::Value { start, end: index });
                    last_start_index = None;
                }
                vec.push(RawSyntax::Comma);
                continue;
            },
            '(' => {
                if let Some(start) = last_start_index {
                    vec.push(RawSyntax::Function { start, end: index });
                    last_start_index = None;
                }
                vec.push(RawSyntax::Parenthesis(ast::ParenthesisType::Open));
                continue;
            }
                _ => {}
        }

        if let Ok(val) = ParenthesisType::try_from(value) {
            match val {
                ParenthesisType::Close | ParenthesisType::CloseSquare | ParenthesisType::CloseCurly => {
                    if let Some(start) = last_start_index {
                        vec.push(RawSyntax::Value { start, end: index });
                        last_start_index = None;
                    }
                }
                _ => {}
            }
            vec.push(RawSyntax::Parenthesis(val));
            continue;
        }

        if let Ok(val) = Operator::try_from(value) {
            if let Some(start) = last_start_index {
                vec.push(RawSyntax::Value { start, end: index });
                last_start_index = None;
            }
            vec.push(RawSyntax::Operator(val));
            continue;
        }
    }

    Ok(vec)
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
