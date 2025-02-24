use crate::EquationParseError;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
/// An easily parsable equation syntax representation
pub enum RawSyntax {
    /// Indices to a literal value, like `5`
    ValueLit { start: usize, end: usize },
    /// Indices to a value identifier, like `x`
    ValueIdent { start: usize, end: usize },
    /// An operator, like `+`, `-`, `*`, `/`
    Operator(Operator),
    /// A parenthesis, like `(` or `)`
    Parenthesis(ParenthesisType),
    /// A known function name, like `sin` or `cos`
    Function { start: usize, end: usize },
    /// A comma. Only used for functions (like log) that can have more than one argument
    Comma,
    /// Absolute value symbol, |x|
    Abs,
}

#[derive(Debug, Clone, Copy)]
pub enum Syntax<'a, T> {
    /// A literal value, like `5`
    ValueLit(T),
    /// A literal value identifier, like `x`.
    /// Can only be a (case sensitive) letter (or symbol, like pi).
    ValueIdent(&'a str),
    /// An operator, like `+`, `-`, `*`, `/`
    Operator(Operator),
    /// A parenthesis, like `(` or `)`
    Parenthesis(ParenthesisType),
    /// A known function name, like `sin` or `cos`
    Function(FunctionType),
    /// A comma. Only used for functions (like log) that can have more than one argument
    Comma,
    /// Absolute value symbol, |x|
    Abs,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `^`
    Pow,
}

impl TryFrom<char> for Operator {
    type Error = crate::EquationParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            '^' => Ok(Self::Pow),
            _ => Err(EquationParseError::NoMatch),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParenthesisType {
    /// `(`
    Open,
    /// `)`
    Close,
    /// `[`
    OpenSquare,
    /// `]`
    CloseSquare,
    /// `{`
    OpenCurly,
    /// `}`
    CloseCurly,
}

impl TryFrom<char> for ParenthesisType {
    type Error = crate::EquationParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Open),
            ')' => Ok(Self::Close),
            '[' => Ok(Self::OpenSquare),
            ']' => Ok(Self::CloseSquare),
            '{' => Ok(Self::OpenCurly),
            '}' => Ok(Self::CloseCurly),
            _ => Err(EquationParseError::NoMatch),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,

    Log,
    Ln,

    Exp,
    Mod,

    Ceil,
    Floor,
    Round,
    Abs,
}

impl FromStr for FunctionType {
    type Err = EquationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "cot" => Ok(Self::Cot),
            "sec" => Ok(Self::Sec),
            "csc" => Ok(Self::Csc),
            "sinh" => Ok(Self::Sinh),
            "cosh" => Ok(Self::Cosh),
            "tanh" => Ok(Self::Tanh),
            "coth" => Ok(Self::Coth),
            "sech" => Ok(Self::Sech),
            "csch" => Ok(Self::Csch),

            "log" => Ok(Self::Log),
            "ln" => Ok(Self::Ln),

            "exp" => Ok(Self::Exp),
            "mod" => Ok(Self::Mod),

            "ceil" => Ok(Self::Ceil),
            "floor" => Ok(Self::Floor),
            "round" => Ok(Self::Round),
            "abs" => Ok(Self::Abs),
            _ => Err(EquationParseError::UnknownFunction),
        }
    }
}
