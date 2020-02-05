use nom::{
    branch::alt,
    bytes::complete::take_until,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, digit1, line_ending, one_of, space1, tab},
    combinator::cut,
    error::{context, ParseError},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::types::Date;

fn tab_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context("indent to ignore (tab)", tab)(input).map(|(remaining, _)| (remaining, ()))
}
fn four_space_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context("indent to ignore (space)", tag("    "))(input).map(|(remaining, _)| (remaining, ()))
}

pub fn indent_tab_or_4_space<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (), E> {
    context("indent to ignore", alt((tab_hole, four_space_hole)))(input)
}

pub fn date<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Date, E> {
    let (input, (day, _, month, _, year)) =
        context("date", tuple((integer, space1, integer, space1, integer)))(input)?;
    Ok((input, Date { day, month, year }))
}

fn parse_str<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alt((alphanumeric1, space1)), '\\', one_of("\"n\\"))(input)
}

pub fn string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "\"-delimited string",
        alt((
            preceded(char('"'), cut(terminated(parse_str, char('"')))),
            alphanumeric1,
            preceded(char('`'), cut(terminated(take_until("`"), char('`')))),
        )),
    )(input)
}

pub fn integer<'a, T: std::str::FromStr, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, T, E>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    context("integer", digit1)(input).map(|(input, value)| (input, value.parse::<T>().unwrap()))
}

pub fn comment_hole<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    context(
        "comment to ignore",
        tuple((tag("// "), terminated(take_until("\n"), line_ending))),
    )(input)
    .map(|(remaining, _)| (remaining, ()))
}

/// helper to build function that will parse a field with an indententation
#[macro_export]
macro_rules! parse_item_with_indent {
    ($nb_ident:expr, $fn_name:ident, $tag:ident, $subparser:ident, $result:ty) => {
        fn $fn_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, $result, E> {
            let (input, (_indent, _tag, _ws, extracted, _newline)) = context(
                stringify!($tag),
                tuple((
                    count(indent_tab_or_4_space, $nb_ident),
                    tag(stringify!($tag)),
                    space1,
                    $subparser,
                    line_ending,
                )),
            )(input)?;

            Ok((input, extracted))
        }
    };
}
