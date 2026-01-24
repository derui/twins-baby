use std::error::Error;

use anyhow::Result;
use nom::{
    AsChar, Finish, IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace0, one_of},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::{many0, many1},
    number::complete::recognize_float,
    sequence::delimited,
};

use crate::equation::{
    Equation,
    arithmetic::{ArithmeticEquation, Operator},
    monomial::MonomialEquation,
};

#[derive(Debug, Clone)]
enum Syntax {
    // direct representation for constant/monomial
    Constant(Equation),
    Monomial(Equation),
    // operator should be construct with a equation
    WithOp(Operator, Box<Syntax>),
    Paren(Vec<Syntax>),
}

fn number(input: &str) -> IResult<&str, f32> {
    let (input, f) = recognize_float(input)?;
    let v: f32 = f.parse().expect("should be parsable");

    Ok((input, v))
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUM: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Parse a variable name
fn variable(input: &str) -> IResult<&str, String> {
    // variable name as
    let name = (ALPHANUM.to_string() + "_").to_owned();
    let char = one_of(name.as_str());
    let mut parser = recognize((one_of(ALPHA), many0(char)));

    let (input, v): (&str, &str) = parser.parse(input)?;
    Ok((input, v.to_string()))
}

/// Parse an operator
fn op(input: &str) -> IResult<&str, Operator> {
    map(one_of("-+*/"), |v| match v {
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
        '-' => Operator::Subtract,
        '+' => Operator::Add,
        _ => unreachable!("do not come here!"),
    })
    .parse(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

fn signed_digit(input: &str) -> IResult<&str, i32> {
    let (input, sign) = opt(one_of("+-")).parse(input)?;
    let (input, digit) = take_while1(|c: char| c.is_dec_digit()).parse(input)?;

    let mut value: i32 = digit.parse().expect("should be parsable");
    if let Some(s) = sign
        && s == '-'
    {
        value = -value;
    }

    Ok((input, value))
}

/// Parse a monomial equation
fn monomial(input: &str) -> IResult<&str, Syntax> {
    let coeff = opt(number);
    let exponent = map(opt((char('^'), signed_digit)), |v| v.map(|v| v.1));

    let (input, (coeff, var, exp)) = (coeff, ws(variable), exponent).parse(input)?;

    Ok((
        input,
        Syntax::Monomial(
            MonomialEquation::new(coeff.unwrap_or(1.0), &var, exp.unwrap_or(1_i32)).into(),
        ),
    ))
}

fn constant(input: &str) -> IResult<&str, Syntax> {
    let (input, value) = ws(number).parse(input)?;

    Ok((input, Syntax::Constant(value.into())))
}

/// parse (...) equation
fn paren_syntax(input: &str) -> IResult<&str, Syntax> {
    let lparen = ws(char('('));
    let rparen = ws(char(')'));
    let mut eq = delimited(lparen, (equation, many0(equation_with_op)), rparen);

    let (input, (op, ops)) = eq.parse(input)?;
    let ret = vec![vec![op], ops].into_iter().flatten().collect();

    Ok((input, Syntax::Paren(ret)))
}

/// Parse equation from string.
fn equation(input: &str) -> IResult<&str, Syntax> {
    alt((paren_syntax, monomial, constant)).parse(input)
}

/// Parse equation from string.
fn equation_with_op(input: &str) -> IResult<&str, Syntax> {
    let (input, (op, syntax)) = (ws(op), equation).parse(input)?;

    Ok((input, Syntax::WithOp(op, Box::new(syntax))))
}

/// Construct an equation with parsed syntaxs.
fn construct_equation(syntax: &[Syntax]) -> Result<Equation> {
    let mut current: Equation = match syntax.first() {
        Some(Syntax::Constant(v)) => v.clone(),
        Some(Syntax::Monomial(v)) => v.clone(),
        Some(Syntax::Paren(v)) => construct_equation(&v)?,
        Some(Syntax::WithOp(_, _)) => unreachable!("This case is parse error"),
        None => unreachable!("Must be able to get first syntax"),
    };
    let mut index = 1;

    while index < syntax.len() {
        let next = &syntax[index];

        current = match next {
            Syntax::Constant(equation) => unreachable!("This case is parse error : {}", equation),
            Syntax::Monomial(equation) => unreachable!("This case is parse error : {}", equation),
            Syntax::WithOp(operator, syntax) => ArithmeticEquation::new(
                *operator,
                &vec![current, construct_equation(&[*syntax.clone()])?],
            )
            .map(|v| Equation::Arithmetic(v))?,
            Syntax::Paren(items) => unreachable!("This case is parse error : {:?}", items),
        };

        index += 1;
    }

    Ok(current)
}

/// Parse an equation from input string
///
/// # Arguments
/// * `input` - A string slice that holds the equation
///
/// # Returns
/// * `Result<Equation, Box<dyn Error>>` - Parsed Equation or an error
pub fn parse(input: &str) -> Result<Equation, Box<dyn Error + '_>> {
    let (rest, syntax) = equation(input)?;
    let (rest, rest_syntax) = many0(equation_with_op)
        .parse(rest)
        .finish()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    if !rest.trim().is_empty() {
        return Err(format!("Unparsed input remaining: {}", rest).into());
    }

    let syntaxes: Vec<Syntax> = vec![vec![syntax], rest_syntax]
        .into_iter()
        .flatten()
        .collect();
    let eq = construct_equation(&syntaxes)?;

    Ok(eq)
}

#[cfg(test)]
mod tests;
