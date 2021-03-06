use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::opt,
    error::context,
    multi::separated_list,
    number::complete::float,
    sequence::{preceded, tuple},
    IResult,
};

use crate::helpers::{indent, integer, integer_i32, integer_u32, resource_path, string};
use crate::types::{Outfit, Weapon};
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
            energy_capacity,
            "\"energy capacity\"",
            integer_u32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            solar_collection,
            "\"solar collection\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            energy_generation,
            "\"energy generation\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            flare_sprite,
            "\"flare sprite\"",
            |input| crate::ship::parse_sprite(0, input),
            input,
            builder
        );
        crate::parse_item_in_loop!(1, flare_sound, "\"flare sound\"", string, input, builder);
        crate::parse_item_in_loop!(1, gun_ports, "\"gun ports\"", float, input, builder);
        crate::parse_item_in_loop!(
            1,
            turret_mounts,
            "\"turret mounts\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(1, weapon, parse_weapon, input, builder);
        crate::parse_item_in_loop!(1, ammo, string, input, builder);
        crate::parse_item_in_loop!(
            1,
            gatling_round_capacity,
            "\"gatling round capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            javelin_capacity,
            "\"javelin capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            meteor_capacity,
            "\"meteor capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            rocket_capacity,
            "\"rocket capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            sidewinder_capacity,
            "\"sidewinder capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            torpedo_capacity,
            "\"torpedo capacity\"",
            integer_i32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            1,
            typhoon_capacity,
            "\"typhoon capacity\"",
            integer_i32,
            input,
            builder
        );

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

#[allow(clippy::cognitive_complexity)]
pub fn parse_weapon<'a>(input: &'a str) -> IResult<&'a str, Weapon<'a>, DataError<&'a str>> {
    let (input, _) = line_ending(input)?;

    let mut builder = crate::types::WeaponBuilder::default();
    let mut input = input;
    loop {
        crate::parse_item_in_loop!(
            2,
            sprite,
            |input| crate::ship::parse_sprite(1, input),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            hardpoint_sprite,
            "\"hardpoint sprite\"",
            |input| crate::ship::parse_sprite(1, input),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            hardpoint_offset,
            "\"hardpoint offset\"",
            separated_list(space1, float),
            input,
            builder
        );
        crate::parse_item_in_loop!(2, sound, string, input, builder);
        crate::parse_item_in_loop!(2, ammo, string, input, builder);
        crate::parse_item_in_loop!(2, icon, resource_path, input, builder);
        crate::parse_item_in_loop!(
            2,
            hit_effect,
            "\"hit effect\"",
            tuple((string, opt(preceded(space1, integer_i32)))),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            fire_effect,
            "\"fire effect\"",
            tuple((string, opt(preceded(space1, integer_i32)))),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            die_effect,
            "\"die effect\"",
            tuple((string, opt(preceded(space1, integer_i32)))),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            submunition,
            "\"submunition\"",
            tuple((string, opt(preceded(space1, integer_i32)))),
            input,
            builder
        );
        crate::parse_item_in_loop!(2, anti_missile, "\"anti-missile\"", float, input, builder);
        crate::parse_item_in_loop!(2, inaccuracy, "\"inaccuracy\"", float, input, builder);
        crate::parse_item_in_loop!(2, turret_turn, "\"turret turn\"", float, input, builder);
        crate::parse_item_in_loop!(2, velocity, "\"velocity\"", float, input, builder);
        crate::parse_item_in_loop!(2, lifetime, "\"lifetime\"", float, input, builder);
        crate::parse_item_in_loop!(
            2,
            random_velocity,
            "\"random velocity\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            random_lifetime,
            "\"random lifetime\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, reload, "\"reload\"", float, input, builder);
        crate::parse_item_in_loop!(2, firing_energy, "\"firing energy\"", float, input, builder);
        crate::parse_item_in_loop!(2, firing_force, "\"firing force\"", float, input, builder);
        crate::parse_item_in_loop!(2, firing_fuel, "\"firing fuel\"", float, input, builder);
        crate::parse_item_in_loop!(2, firing_heat, "\"firing heat\"", float, input, builder);
        crate::parse_item_in_loop!(2, hit_force, "\"hit force\"", float, input, builder);
        crate::parse_item_in_loop!(2, shield_damage, "\"shield damage\"", float, input, builder);
        crate::parse_item_in_loop!(2, hull_damage, "\"hull damage\"", float, input, builder);
        crate::parse_item_in_loop!(2, heat_damage, "\"heat damage\"", float, input, builder);
        crate::parse_item_in_loop!(2, acceleration, "\"acceleration\"", float, input, builder);
        crate::parse_item_in_loop!(2, drag, "\"drag\"", float, input, builder);
        crate::parse_item_in_loop!(2, turn, "\"turn\"", float, input, builder);
        crate::parse_item_in_loop!(2, homing, "\"homing\"", float, input, builder);
        crate::parse_item_in_loop!(
            2,
            infrared_tracking,
            "\"infrared tracking\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            radar_tracking,
            "\"radar tracking\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            optical_tracking,
            "\"optical tracking\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            trigger_radius,
            "\"trigger radius\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, blast_radius, "\"blast radius\"", float, input, builder);
        crate::parse_item_in_loop!(
            2,
            missile_strength,
            "\"missile strength\"",
            float,
            input,
            builder
        );
        crate::parse_item_in_loop!(2, stream, |input| Ok((input, true)), input, builder);
        crate::parse_item_in_loop!(
            2,
            cluster,
            "\"cluster\"",
            |input| Ok((input, true)),
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            burst_count,
            "\"burst count\"",
            integer_u32,
            input,
            builder
        );
        crate::parse_item_in_loop!(
            2,
            burst_reload,
            "\"burst reload\"",
            integer_u32,
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
                data_type: String::from("weapon"),
            })
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_outfit() {
        let data = r#"outfit "My Little Engine"
    category "Engines"
    "cost" 20000
    thumbnail "outfit/little engines"
    "mass" 20
    "outfit space" -20
    "engine capacity" -20
    "turn" 100
    "turning energy" .2
    "turning heat" .5
    "thrust" 4.0
    "thrusting energy" .5
    "thrusting heat" .5
    "flare sprite" "effect/flare/v"
        "frame rate" 1.2
    "flare sound" "little"
    description `It's a little engine that could`
"#;

        let parsed = dbg!(parse_outfit(&data));
        assert!(parsed.is_ok());
        let result = parsed.unwrap();
        assert_eq!(result.0, "");

        let engine = result.1;
        assert_eq!(engine.name, "My Little Engine");
    }
}
