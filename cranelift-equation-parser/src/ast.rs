use crate::EquationParseError;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
/// An easily parsable and cheaply clonable 1st stage AST.
/// This is a direct mapping of the equation text to something we can work with.
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
/// Easily parsable 2nd stage AST.
/// At this stage, the parser has expanded out implicit multiplication, parsed literals and functions,
/// but has no idea about the relationship between symbols.
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

    Sqrt,
    Root,

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

            "sqrt" => Ok(Self::Sqrt),
            "root" => Ok(Self::Root),

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

#[derive(Debug, Clone, Copy)]
/// A cheaply clonable high level 3rd stage AST.
/// At this stage, the parser has fully thought out the relationship between symbols.
pub enum Entity<'a, T> {
    ValueLit(T),
    Value(&'a str),
    Operation(Operation<'a, T>),
    Function(Function<'a, T>)
}

#[derive(Debug, Clone, Copy)]
pub enum Operation<'a, T> {
    Add(&'a Entity<'a, T>, &'a Entity<'a, T>),
    Sub(&'a Entity<'a, T>, &'a Entity<'a, T>),
    Mul(&'a Entity<'a, T>, &'a Entity<'a, T>),
    Div(&'a Entity<'a, T>, &'a Entity<'a, T>),
    Pow(&'a Entity<'a, T>, &'a Entity<'a, T>),
}

#[derive(Debug, Clone, Copy)]
pub enum Function<'a, T> {
    Sin(&'a Entity<'a, T>),
    Cos(&'a Entity<'a, T>),
    Tan(&'a Entity<'a, T>),
    Cot(&'a Entity<'a, T>),
    Sec(&'a Entity<'a, T>),
    Csc(&'a Entity<'a, T>),
    Sinh(&'a Entity<'a, T>),
    Cosh(&'a Entity<'a, T>),
    Tanh(&'a Entity<'a, T>),
    Coth(&'a Entity<'a, T>),
    Sech(&'a Entity<'a, T>),
    Csch(&'a Entity<'a, T>),

    Log(&'a Entity<'a, T>, &'a Entity<'a, T>),
    Log10(&'a Entity<'a, T>),
    Ln(&'a Entity<'a, T>),

    Sqrt(&'a Entity<'a, T>),
    Root(&'a Entity<'a, T>, &'a Entity<'a, T>),

    Exp(&'a Entity<'a, T>),
    Mod(&'a Entity<'a, T>, &'a Entity<'a, T>),

    Ceil(&'a Entity<'a, T>),
    Floor(&'a Entity<'a, T>),
    Round(&'a Entity<'a, T>),
    Abs(&'a Entity<'a, T>),
}