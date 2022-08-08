use std::num::ParseIntError;

/// An error which can be returned.
#[derive(Debug)]
pub enum Error {
    /// App started without input expression.
    ExpressionNotProvided,
    /// Invalid input with explanation.
    InvalidInput(String),
    /// Int number parsing error.
    Parsing(ParseIntError),
}
