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

fn tab_hole(input: &str) -> IResult<&str, ()> {
    tab(input).map(|(remaining, _)| (remaining, ()))
}
fn four_space_hole(input: &str) -> IResult<&str, ()> {
    tag("    ")(input).map(|(remaining, _)| (remaining, ()))
}

pub fn indent_tab_or_4_space(input: &str) -> IResult<&str, ()> {
    alt((tab_hole, four_space_hole))(input)
}

pub fn date(input: &str) -> IResult<&str, Date> {
    let (input, (day, _, month, _, year)) =
        context("date", tuple((integer, space1, integer, space1, integer)))(input)?;
    Ok((
        input,
        Date {
            day: day,
            month: month,
            year: year,
        },
    ))
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    escaped(alt((alphanumeric1, space1)), '\\', one_of("\"n\\"))(input)
}

pub fn string(input: &str) -> IResult<&str, &str> {
    context(
        "string",
        alt((
            preceded(char('"'), cut(terminated(parse_str, char('"')))),
            alphanumeric1,
            preceded(char('`'), cut(terminated(take_until("`"), char('`')))),
        )),
    )(input)
}

pub fn integer<T: std::str::FromStr>(input: &str) -> IResult<&str, T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    digit1(input).map(|(input, value)| (input, value.parse::<T>().unwrap()))
}

pub fn comment_hole(input: &str) -> IResult<&str, ()> {
    tuple((tag("// "), terminated(take_until("\n"), line_ending)))(input)
        .map(|(remaining, _)| (remaining, ()))
}

#[macro_export]
macro_rules! parse_item_with_indent {
    ($nb_ident:expr, $fn_name:ident, $tag:ident, $subparser:ident, $result:ty) => {
        fn $fn_name(input: &str) -> IResult<&str, $result> {
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
