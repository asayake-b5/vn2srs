use nom::{
    character::complete::{char, one_of},
    character::complete::{line_ending, multispace0, not_line_ending},
    combinator::recognize,
    error::ParseError,
    multi::{many0, many1},
    sequence::delimited,
    sequence::terminated,
    IResult, Parser,
};

pub fn p_decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

///Discards the line ending, returning the input of that line and the rest
pub fn discard_line_ending(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

pub fn number_around_spaces(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, p_decimal, multispace0)(input)
}
