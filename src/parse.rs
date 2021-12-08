use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple ,
  sequence::preceded ,
   error::ParseError,
  combinator::value,
  sequence::delimited,
  sequence::separated_pair,
  character::complete::multispace0,
  character::complete::digit1,
  character::complete::alpha1,
  number::complete::be_i32,
  branch::alt,
  multi::many0,
};



use crate::ast::AExp;
use crate::ast::BExp;

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
  delimited(
    multispace0,
    delimited(tag("("), aexpr, tag(")")),
    multispace0
  )(input)
}

fn var(input: &str) -> IResult<&str, AExp> {
    let (input, var) = delimited(
      multispace0,
      alpha1,
      multispace0
    )(input)?;
    Ok((input, AExp::Id(var.to_string())))
}
fn int(input: &str) -> IResult<&str, AExp> {
    // let (input, dig) = nom::number::complete::be_i32(input)?;
    // Ok((input, AExp::Int(dig)))
    let (input, num) = delimited(
      multispace0,
      digit1,
      multispace0
    )(input)?;
    Ok((input, AExp::Int(num.parse().unwrap())))
}

fn not_bin(input: &str) -> IResult<&str, AExp> {
    (alt((parenth, int, var)))(input)
}
fn div_extra(input: &str) -> IResult<&str, Vec<AExp>> {
    many0(preceded(tag("/"), not_bin))(input)
}
fn div(input: &str) -> IResult<&str, AExp> {
    // let (input, exp1) = alt(parenth, todo!());
    let (input, (init, extra)) = tuple((not_bin, div_extra))(input)?;
    Ok((input, extra.into_iter().fold(init, |acc, x| AExp::Divide(Box::new(acc), Box::new(x)))))
}

// plus ::= plus + div_exp | div_exp
// plus ::= div_exp plus'
// plus' ::= (+div_exp) plus' | nothing

fn plus_extra(input: &str) -> IResult<&str, Vec<AExp>> {
    many0(preceded(tag("+"), div))(input)
}
fn plus(input: &str) -> IResult<&str, AExp> {
    let (input, (init, extra)) = tuple((div, plus_extra))(input)?;
    Ok((input, extra.into_iter().fold(init, |acc, x| AExp::Plus(Box::new(acc), Box::new(x)))))
}
fn aexpr(input: &str) -> IResult<&str, AExp> {
    plus(input)
}



fn ltexp(input: &str) -> IResult<&str, BExp> {
    let (input, (left, right)) = separated_pair(aexpr, tag("<="), aexpr)(input)?;
    Ok((input, (BExp::LessThanEq(Box::new(left), Box::new(right)))))
}
fn and_extra(input: &str) -> IResult<&str, Vec<BExp>> {
    many0(preceded(tag("&&"), ltexp))(input)
}

// bexp ::= bexp && ltexp | ltexp | ( bexp )
// plus ::= div_exp plus'
// plus' ::= (+div_exp) plus' | nothing
fn bexp(input: &str) -> IResult<&str, BExp> {
    //let (input, (init, extra)) = alt((tuple((ltexp, and_extra))))(input)?;
    //Ok((input, extra.into_iter().fold(init, |acc, x| BExp::And(Box::new(acc), Box::new(x)))))
    todo!()
}