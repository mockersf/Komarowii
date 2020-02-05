use nom::{
    branch::alt,
    bytes::complete::take_until,
    bytes::complete::{escaped, tag},
    character::complete::{alphanumeric1, char, digit1, line_ending, one_of, space1, tab},
    combinator::cut,
    error::context,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::types::Date;

fn tab_hole<'a>(input: &'a str) -> IResult<&'a str, ()> {
    tab(input).map(|(remaining, _)| (remaining, ()))
}
fn four_space_hole<'a>(input: &'a str) -> IResult<&'a str, ()> {
    tag("    ")(input).map(|(remaining, _)| (remaining, ()))
}

pub fn indent_tab_or_4_space<'a>(input: &'a str) -> IResult<&'a str, ()> {
    alt((tab_hole, four_space_hole))(input)
}

pub fn date<'a>(input: &'a str) -> IResult<&'a str, Date> {
    let (input, (day, _, month, _, year)) =
        context("date", tuple((integer, space1, integer, space1, integer)))(input)?;
    Ok((input, Date { day, month, year }))
}

fn parse_str<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    escaped(alt((alphanumeric1, space1)), '\\', one_of("\"n\\"))(input)
}

pub fn string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    context(
        "string",
        alt((
            preceded(char('"'), cut(terminated(parse_str, char('"')))),
            alphanumeric1,
            preceded(char('`'), cut(terminated(take_until("`"), char('`')))),
        )),
    )(input)
}

pub fn integer<'a, T: std::str::FromStr>(input: &'a str) -> IResult<&'a str, T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    digit1(input).map(|(input, value)| (input, value.parse::<T>().unwrap()))
}

pub fn comment_hole<'a>(input: &'a str) -> IResult<&'a str, ()> {
    tuple((tag("// "), terminated(take_until("\n"), line_ending)))(input)
        .map(|(remaining, _)| (remaining, ()))
}

#[macro_export]
macro_rules! parse_item_with_indent {
    ($nb_ident:expr, $fn_name:ident, $tag:ident, $subparser:ident, $result:ty) => {
        fn $fn_name<'a>(input: &'a str) -> IResult<&'a str, $result> {
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
