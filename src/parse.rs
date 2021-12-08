use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple ,
  sequence::preceded ,
   error::ParseError,
  combinator::value,
  sequence::delimited,
  character::complete::multispace0,
  character::complete::digit1,
  number::complete::i32;
};

use crate::ast::AExp;

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

fn parenth(input: &str) -> IResult<&str, AExp> {
    delimited(tag("("), aexpr, tag(")"))(input)
}

fn int(input: &str) -> IResult<&str, AExp> {
    let (input, dig) = nom::number::complete::i32(input)?;
    Ok((input, AExp::Int(dig)))
}
fn div(input: &str) -> IResult<&str, AExp> {
    let (input, exp1) = alt(parenth, todo!());
    todo!()
}
fn maybe_plus(input: &str) -> IResult<&str, Option<AExp>> {
    match preceded(tag("+"), plus)(input) {
        Ok((input, x)) => Ok((input, Some(x))),
        Err(_) => Ok((input, None))
    }
}
fn plus(input: &str) -> IResult<&str, AExp> {
    let (input, (exp1, exp2)) = tuple((div, maybe_plus))(input)?;
    match exp2 {
        Some(x) => Ok((input, AExp::Plus(Box::new(exp1), Box::new(x)))),
        None => Ok((input, exp1))
    }
}
fn aexpr(input: &str) -> IResult<&str, AExp> {
    let exp1 = plus(input)?;
    todo!()

}
