mod error;
mod expression;
mod number;

#[cfg(test)]
mod tests;

use std::env::args;

use crate::{error::Error, expression::Expression};

fn main() -> Result<(), Error> {
    let input = args().nth(1).ok_or(Error::ExpressionNotProvided)?;
    // println!("{input:?}");

    let exp = Expression::try_from(&input)?;

    let output = exp.evaluate().map(|n| n.0)?;

    println!("{input} = {output}");

    Ok(())
}
