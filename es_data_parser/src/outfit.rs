use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    error::context,
    number::complete::float,
    sequence::tuple,
    IResult,
};

use crate::helpers::{indent, integer, resource_path, string};
use crate::types::Outfit;
use crate::DataError;

#[allow(clippy::cognitive_complexity)]
pub fn parse_outfit<'a>(input: &'a str) -> IResult<&'a str, Outfit<'a>, DataError<&'a str>> {
    let (input, (_, _, name, _)) = context(
        "outfit tag",
        tuple((tag("outfit"), space1, string, line_ending)),
    )(input)?;

    let mut builder = crate::types::OutfitBuilder::default();
    builder.name(name);
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(1, plural, string, input, builder);
        crate::parse_item_in_loop!(1, category, string, input, builder);
        crate::parse_item_in_loop!(1, cost, integer, input, builder);
        crate::parse_item_in_loop!(1, cost, "\"cost\"", integer, input, builder);
        crate::parse_item_in_loop!(1, unplunderable, integer, input, builder);
        crate::parse_item_in_loop!(
            1,
            unplunderable,
            "\"unplunderable\"",
            integer,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, thumbnail, resource_path, input, builder);
        crate::parse_item_in_loop!(1, mass, "\"mass\"", float, input, builder);
        crate::parse_item_in_loop!(1, outfit_space, "\"outfit space\"", float, input, builder);
        crate::parse_item_in_loop!(1, cargo_space, "\"cargo space\"", float, input, builder);
        crate::parse_item_in_loop!(1, cooling, "\"cooling\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            cooling_inefficiency,
            "\"cooling inefficiency\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            heat_dissipation,
            "\"heat dissipation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            shield_generation,
            "\"shield generation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, shield_energy, "\"shield energy\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            energy_consumption,
            "\"energy consumption\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            heat_generation,
            "\"heat generation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, radar_jamming, "\"radar jamming\"", float, input, builder);
        crate::parse_item_in_loop!(1, ramscoop, "\"ramscoop\"", float, input, builder);
        crate::parse_item_in_loop!(1, jump_speed, "\"jump speed\"", float, input, builder);
        crate::parse_item_in_loop!(1, jump_fuel, "\"jump fuel\"", float, input, builder);
        crate::parse_item_in_loop!(1, hyperdrive, "\"hyperdrive\"", float, input, builder);
        crate::parse_item_in_loop!(1, scram_drive, "\"scram drive\"", float, input, builder);
        crate::parse_item_in_loop!(1, jump_drive, "\"jump drive\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            cargo_scan_power,
            "\"cargo scan power\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            cargo_scan_speed,
            "\"cargo scan speed\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            outfit_scan_power,
            "\"outfit scan power\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            outfit_scan_speed,
            "\"outfit scan speed\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            asteroid_scan_power,
            "\"asteroid scan power\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            tactical_scan_power,
            "\"tactical scan power\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            atmosphere_scan,
            "\"atmosphere scan\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, cloak, "\"cloak\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            cloaking_energy,
            "\"cloaking energy\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, cloaking_fuel, "\"cloaking fuel\"", float, input, builder);
        crate::parse_item_in_loop!(1, bunks, "\"bunks\"", float, input, builder);
        crate::parse_item_in_loop!(1, required_crew, "\"required crew\"", float, input, builder);
        crate::parse_item_in_loop!(1, fuel_capacity, "\"fuel capacity\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            scan_interference,
            "\"scan interference\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            capture_attack,
            "\"capture attack\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            capture_defense,
            "\"capture defense\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, illegal, "\"illegal\"", float, input, builder);
        crate::parse_item_in_loop!(1, map, "\"map\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            weapon_capacity,
            "\"weapon capacity\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            engine_capacity,
            "\"engine capacity\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            afterburner_thrust,
            "\"afterburner thrust\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            afterburner_fuel,
            "\"afterburner fuel\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            afterburner_energy,
            "\"afterburner energy\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            afterburner_heat,
            "\"afterburner heat\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            afterburner_effect,
            "\"afterburner effect\"",
            string,
            input,
            builder
        );

        crate::parse_item_in_loop!(1, turn, "\"turn\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            turning_energy,
            "\"turning energy\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, turning_heat, "\"turning heat\"", float, input, builder);
        crate::parse_item_in_loop!(1, thrust, "\"thrust\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            thrusting_energy,
            "\"thrusting energy\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            thrusting_heat,
            "\"thrusting heat\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            reverse_thrust,
            "\"reverse thrust\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            reverse_thrusting_energy,
            "\"reverse thrusting energy\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            reverse_thrusting_heat,
            "\"reverse thrusting heat\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            flare_sprite,
            "\"flare sprite\"",
            crate::ship::parse_sprite,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, flare_sound, "\"flare sound\"", string, input, builder);

        crate::parse_items_in_loop!(1, description, string, input, builder);

        break;
    }

    builder
        .build()
        .map(|outfit| (input, outfit))
        .map_err(|error| {
            nom::Err::Failure(DataError::DataBuilderError {
                input,
                error,
                data_type: String::from("outfit"),
            })
        })
}
