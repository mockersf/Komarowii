use std::sync::Arc;

use rand::seq::IteratorRandom;

use super::{Game, Object, Player, Ship, System};

/// Helper to load es data files and create a game
#[derive(Debug)]
pub struct ESGameLoader {
    ships: Vec<Ship>,
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

#[allow(clippy::new_without_default)]
impl ESGameLoader {
    /// Start an empty es game loader
    pub fn empty() -> Self {
        Self {
            ships: vec![],
            systems: vec![],
            start: None,
        }
    }

    /// Load a game file
    pub fn load(&mut self, es_game_data_source: &str) {
        let es_game_data = es_data_parser::parse(es_game_data_source);

        let mut ships = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::Ship(ship) = object {
                    Some(ship)
                } else {
                    None
                }
            })
            .map(|ship| Ship {
                name: String::from(ship.name),
                sprite: match ship.sprite {
                    es_data_parser::Sprite::Simple(sprite) => String::from(sprite),
                    es_data_parser::Sprite::Sprite { name, .. } => format!("{}=0", name),
                },
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

    /// Create a game from the loaded files
    pub fn create_game(&self) -> Result<Game, ()> {
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

        Ok(Game {
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
