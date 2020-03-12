use std::sync::Arc;

use rand::seq::IteratorRandom;

use super::data::*;

/// resolved game data
#[derive(Debug)]
pub struct ESGameLoader {
    outfits: Vec<Outfit>,
    ships: Vec<Ship>,
    systems: Vec<System>,
    start: Option<(String, (i32, u32, u32))>,
}

/// Helper to load es data files and create a game
#[derive(Debug)]
pub struct UnresolvedESGameLoader {
    outfits: Vec<Outfit>,
    ships: Vec<super::unresolved_data::Ship>,
    systems: Vec<System>,
    start: Option<(String, (i32, u32, u32))>,
}

fn es_object_to_object<'a>(object: &es_data_parser::SystemObject<'a>) -> Object {
    Object {
        sprite: object.sprite.map(String::from),
        distance: object.distance.unwrap_or(0.0),
        period: object.period,
        objects: object.objects.iter().map(es_object_to_object).collect(),
    }
}

fn outfit_as_engine(outfit: &es_data_parser::Outfit) -> Vec<OutfitEngine> {
    let mut engines = vec![];
    if outfit.thrust.is_some() {
        engines.push(OutfitEngine {
            ty: EngineType::Thrust,
            power: outfit.thrust.unwrap_or(0.0),
            energy_consumption: outfit.energy_consumption.unwrap_or(0.0),
            heat_generation: outfit.heat_generation.unwrap_or(0.0),
        })
    }
    if outfit.turn.is_some() {
        engines.push(OutfitEngine {
            ty: EngineType::Turn,
            power: outfit.turn.unwrap_or(0.0),
            energy_consumption: outfit.turning_energy.unwrap_or(0.0),
            heat_generation: outfit.turning_heat.unwrap_or(0.0),
        })
    }
    if outfit.reverse_thrust.is_some() {
        engines.push(OutfitEngine {
            ty: EngineType::ReverseThrust,
            power: outfit.reverse_thrust.unwrap_or(0.0),
            energy_consumption: outfit.reverse_thrusting_energy.unwrap_or(0.0),
            heat_generation: outfit.reverse_thrusting_heat.unwrap_or(0.0),
        })
    }
    engines
}

#[allow(clippy::new_without_default)]
impl<'a> UnresolvedESGameLoader {
    /// Start an empty es game loader
    pub fn empty() -> Self {
        Self {
            outfits: vec![],
            ships: vec![],
            systems: vec![],
            start: None,
        }
    }

    /// Load a game file
    pub fn load(&mut self, es_game_data_source: &str) {
        let es_game_data = es_data_parser::parse(es_game_data_source);

        let mut outfits = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::Outfit(outfit) = object {
                    Some(outfit)
                } else {
                    None
                }
            })
            .map(|outfit| Outfit {
                name: String::from(outfit.name),
                category: match outfit.category {
                    Some("Systems") => OutfitCategory::Systems,
                    Some("Hand to Hand") => OutfitCategory::HandToHand,
                    Some("Engines") => OutfitCategory::Engines,
                    _ => OutfitCategory::Unspecified,
                },
                mass: outfit.mass as i32,
                engine: outfit_as_engine(outfit),
            })
            .collect::<Vec<_>>();
        self.outfits.append(&mut outfits);

        let mut ships = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::Ship(ship) = object {
                    Some(ship)
                } else {
                    None
                }
            })
            .map(|ship| super::unresolved_data::Ship {
                name: String::from(ship.name),
                sprite: match ship.sprite {
                    es_data_parser::Sprite::Simple(sprite) => String::from(sprite),
                    es_data_parser::Sprite::Sprite { name, .. } => format!("{}=0", name),
                },
                outfits: ship
                    .outfits
                    .iter()
                    .map(|outfit| outfit.0.to_string())
                    .collect(),
                drag: ship.attributes.drag,
                mass: ship.attributes.mass,
            })
            .collect::<Vec<_>>();
        self.ships.append(&mut ships);

        let mut systems = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::System(system) = object {
                    Some(system)
                } else {
                    None
                }
            })
            .map(|system| System {
                name: String::from(system.name),
                objects: system.objects.iter().map(es_object_to_object).collect(),
            })
            .collect::<Vec<_>>();
        self.systems.append(&mut systems);

        if let Some(start) = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::Start(start) = object {
                    Some(start)
                } else {
                    None
                }
            })
            .next()
        {
            self.start = Some((
                String::from(start.system),
                (start.date.year, start.date.month, start.date.day),
            ));
        }
    }

    pub fn resolve(self) -> ESGameLoader {
        let outfits = self.outfits;
        let ships = self
            .ships
            .into_iter()
            .map(|ship| Ship {
                name: ship.name,
                sprite: ship.sprite,
                drag: ship.drag,
                base_mass: ship.mass,
                outfits: ship
                    .outfits
                    .into_iter()
                    .filter_map(|outfit_name| {
                        outfits.iter().find(|outfit| outfit.name == outfit_name)
                    })
                    .cloned()
                    .collect(),
            })
            .collect();
        ESGameLoader {
            outfits,
            systems: self.systems,
            start: self.start,
            ships,
        }
    }
}

impl ESGameLoader {
    /// Create a game from the loaded files
    pub fn create_game(&self) -> Result<super::Game, ()> {
        let mut rng = rand::thread_rng();

        if self.ships.is_empty() {
            return Err(());
        }
        let ships: Vec<Arc<Ship>> = self.ships.iter().cloned().map(Arc::new).collect();

        if self.systems.is_empty() {
            return Err(());
        }
        let systems: Vec<Arc<System>> = self.systems.iter().cloned().map(Arc::new).collect();

        let (start_system, start_date) = if let Some(start) = self.start.clone() {
            let (y, m, d) = start.1;
            (
                systems
                    .iter()
                    .find(|system| system.name == start.0)
                    .ok_or(())?
                    .clone(),
                chrono::NaiveDate::from_ymd_opt(y, m, d).ok_or(())?,
            )
        } else {
            (
                systems.iter().choose(&mut rng).ok_or(())?.clone(),
                chrono::NaiveDate::from_ymd(2020, 1, 1),
            )
        };
        let start_ship = ships.iter().choose(&mut rng).ok_or(())?.clone();

        Ok(super::Game {
            current_date: start_date,
            ships,
            systems,
            player: Player {
                current_system: start_system,
                ship: start_ship,
            },
        })
    }
}
