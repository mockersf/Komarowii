use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space1, tab},
    error::context,
    multi::{count, many1},
    number::complete::float,
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent_tab_or_4_space, integer, string};
use crate::types::{Fleet, Planet, Tribute};

pub fn parse_planet<'a>(input: &'a str) -> IResult<&'a str, Planet<'a>> {
    let (input, (_, _, name, _)) = tuple((tag("planet"), space1, string, line_ending))(input)?;
    let (input, (description, spaceport, shipyard, outfitter, bribe, security, tribute)) =
        permutation((
            many1(parse_description),
            many1(parse_spaceport),
            many1(parse_shipyard),
            many1(parse_outfitter),
            parse_bribe,
            parse_security,
            parse_tribute,
        ))(input)?;

    Ok((
        input,
        Planet {
            name,
            description,
            spaceport,
            shipyard,
            outfitter,
            bribe,
            security,
            tribute,
        },
    ))
}

fn parse_tribute<'a>(input: &'a str) -> IResult<&'a str, Tribute<'a>> {
    let (input, (_, _, _, value, _)) =
        tuple((tab, tag("tribute"), space1, integer, line_ending))(input)?;

    let (input, (threshold, fleet)) = permutation((parse_threshold, parse_fleet))(input)?;

    Ok((
        input,
        Tribute {
            threshold,
            value,
            fleet,
        },
    ))
}

fn parse_fleet<'a>(input: &'a str) -> IResult<&'a str, Fleet<'a>> {
    let (input, (_, _, _, _, kind, _, count)) =
        tuple((tab, tab, tag("fleet"), space1, string, space1, integer))(input)?;

    Ok((input, Fleet { kind, count }))
}

crate::parse_item_with_indent!(1, parse_description, description, string, &'a str);
crate::parse_item_with_indent!(1, parse_spaceport, spaceport, string, &'a str);
crate::parse_item_with_indent!(1, parse_shipyard, shipyard, string, &'a str);
crate::parse_item_with_indent!(1, parse_outfitter, outfitter, string, &'a str);
crate::parse_item_with_indent!(1, parse_bribe, bribe, float, f32);
crate::parse_item_with_indent!(1, parse_security, security, float, f32);

crate::parse_item_with_indent!(2, parse_threshold, threshold, integer, u32);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_parse_planet() {
        let data = r#"planet MyPlanet
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
"#;

        let parsed = dbg!(parse_planet(&data));
        assert!(parsed.is_ok());
        let planet = parsed.unwrap().1;

        assert_eq!(planet.name, "MyPlanet");
        assert_eq!(
            planet
                .description
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
                .join("\n"),
            String::from(
                r#"This is a "special" planet
	It can have a complete description"#
            )
        );
        assert_eq!(
            planet
                .spaceport
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
                .join("\n"),
            String::from("And also a spaceport!")
        );
        assert_eq!(planet.shipyard, vec!["Some Ships", "Also Those Ships"]);
        assert_eq!(planet.outfitter, vec!["Basic Outifts", "Advanced Outfits"]);
        assert_eq!(planet.bribe, 0.01);
        assert_eq!(planet.security, 0.5);
        assert_eq!(
            planet.tribute,
            Tribute {
                threshold: 3000,
                value: 1000,
                fleet: Fleet {
                    kind: "Impressive Fleet",
                    count: 18,
                }
            }
        )
    }
}
