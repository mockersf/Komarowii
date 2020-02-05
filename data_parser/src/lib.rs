use nom::{branch::alt, character::complete::line_ending, combinator::all_consuming, multi::many1};

mod types;
pub use types::*;

mod helpers;

mod planet;
mod start;

pub fn parse<'a>(input: &'a str) -> Result<Vec<Object<'a>>, ()> {
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
    .map_err(|_err| ())
    .map(|(_, data)| data.into_iter().filter_map(|data| data).collect())
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn will_fail_for_empty_input() {
        assert!(parse("").is_err())
    }

    #[test]
    fn can_parse_varied_input() {
        let data = r#"
// my comment

planet MyPlanet
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
        let parsed = dbg!(parse(data));
        assert!(parsed.is_ok());
    }
}
