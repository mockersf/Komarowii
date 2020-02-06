//! Parse Endless Sky data

#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

use nom::{
    branch::alt, character::complete::line_ending, combinator::all_consuming, error::ParseError,
    multi::many1, IResult,
};

mod types;
pub use types::*;

mod helpers;

mod planet;
mod start;

/// Parse Endless Sky data, returning a list of objects parsed or an empty list on error
pub fn parse<'a>(input: &'a str) -> Vec<Object<'a>> {
    validate::<(&str, nom::error::ErrorKind)>(input)
        .map(|(_, data)| data)
        .unwrap_or_else(|_| vec![])
}

/// Parse Endless Sky data
pub fn validate<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Object<'a>>, E> {
    all_consuming(many1(alt((
        |input| {
            start::parse_start(input).map(|(input, parsed)| (input, Some(Object::Start(parsed))))
        },
        |input| {
            planet::parse_planet(input).map(|(input, parsed)| (input, Some(Object::Planet(parsed))))
        },
        |input| line_ending(input).map(|(input, _)| (input, None)),
        |input| helpers::comment_hole(input).map(|(input, _)| (input, None)),
    ))))(input)
    .map(|(remaining, parsed)| {
        (
            remaining,
            parsed.into_iter().filter_map(|object| object).collect(),
        )
    })
}

#[cfg(test)]
mod test {
    use super::validate;

    use nom::error::VerboseError;

    #[test]
    fn will_fail_for_empty_input() {
        assert!(validate::<VerboseError<&str>>("").is_err())
    }

    #[test]
    fn can_parse_varied_input() {
        let data = r#"
// my comment

planet MyPlanet
	attributes a1 a2 a3
	description `This is a "special" planet`
	description `	It can have a complete description`
	spaceport `And also a spaceport!`
	shipyard "Some Ships"
	shipyard "Also Those Ships"
	outfitter "Basic Outifts"
	outfitter "Advanced Outfits"
	bribe 0.01
	security 0.5
	tribute 1000
		threshold 3000
		fleet "Impressive Fleet" 18

start
	system "my system"
	planet "this planet"
    date 01 07 2020
	set "my license"
	account
		credits 5000
		score 100
		mortgage Mortgage
			principal 33333
			interest 0.005
            term 365
"#;
        let parsed = dbg!(validate::<VerboseError<&str>>(data));
        assert!(parsed.is_ok());
    }
}
