use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    error::context,
    number::complete::float,
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent, integer, resource_path, string};
use crate::types::Effect;
use crate::DataError;

pub fn parse_effect<'a>(input: &'a str) -> IResult<&'a str, Effect<'a>, DataError<&'a str>> {
    let (input, (_, _, name, _)) = context(
        "effect tag",
        tuple((tag("effect"), space1, string, line_ending)),
    )(input)?;

    let mut builder = crate::types::EffectBuilder::default();
    builder.name(name);
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(1, sprite, crate::ship::parse_sprite, input, builder);
        crate::parse_item_in_loop!(1, sound, string, input, builder);
        crate::parse_item_in_loop!(1, lifetime, "\"lifetime\"", float, input, builder);
        crate::parse_item_in_loop!(1, random_angle, "\"random angle\"", float, input, builder);
        crate::parse_item_in_loop!(1, random_spin, "\"random spin\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            random_velocity,
            "\"random velocity\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            velocity_scale,
            "\"velocity scale\"",
            float,
            input,
            builder
        );

        break;
    }

    builder
        .build()
        .map(|effect| (input, effect))
        .map_err(|error| {
            nom::Err::Failure(DataError::DataBuilderError {
                input,
                error,
                data_type: String::from("effect"),
            })
        })
}
