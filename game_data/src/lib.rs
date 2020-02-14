use std::sync::Arc;

#[derive(Debug)]
pub struct Ship {
    pub name: String,
    pub sprite: String,
}

#[derive(Debug)]
pub struct Player {
    pub ship: Option<Arc<Ship>>,
    pub current_system: Option<Arc<System>>,
}

#[derive(Debug)]
pub struct Object {
    pub sprite: Option<String>,
    pub distance: f32,
    pub period: f32,
}

#[derive(Debug)]
pub struct System {
    pub name: String,
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Game {
    pub start_system_name: Option<String>,
    pub current_date: Option<chrono::NaiveDate>,
    pub player: Player,
    pub systems: Vec<Arc<System>>,
    pub ships: Vec<Arc<Ship>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Player {
                ship: None,
                current_system: None,
            },
            current_date: None,
            systems: vec![],
            ships: vec![],
            start_system_name: None,
        }
    }

    pub fn get_nb_days_elapsed_since_beginning(&self) -> i64 {
        self.get_nb_days_elapsed_since(chrono::NaiveDate::from_ymd(2020, 1, 1))
    }

    pub fn get_nb_days_elapsed_since(&self, start_date: chrono::NaiveDate) -> i64 {
        (self.current_date.unwrap() - start_date).num_days()
    }

    pub fn add_data_file(&mut self, es_game_data_source: &str) {
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
            .map(|ship| {
                Arc::new(Ship {
                    name: String::from(ship.name),
                    sprite: match ship.sprite {
                        es_data_parser::Sprite::Simple(sprite) => String::from(sprite),
                        es_data_parser::Sprite::Sprite { name, .. } => format!("{}=0", name),
                    },
                })
            })
            .collect::<Vec<_>>();

        let mut systems = es_game_data
            .iter()
            .filter_map(|object| {
                if let es_data_parser::Object::System(system) = object {
                    Some(system)
                } else {
                    None
                }
            })
            .map(|system| {
                Arc::new(System {
                    name: String::from(system.name),
                    objects: system
                        .objects
                        .iter()
                        .map(|object| Object {
                            sprite: object.sprite.map(String::from),
                            distance: object.distance.unwrap_or(0.0),
                            period: object.period,
                        })
                        .collect(),
                })
            })
            .collect::<Vec<_>>();

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
            self.start_system_name = Some(String::from(start.system));
            self.current_date =
                chrono::NaiveDate::from_ymd_opt(start.date.year, start.date.month, start.date.day);
        }

        if self.player.ship.is_none() {
            self.player.ship = ships.iter().find(|ship| ship.name == "Shuttle").cloned();
        }
        self.ships.append(&mut ships);
        self.systems.append(&mut systems);
        if self.player.current_system.is_none() {
            if let Some(system_name) = self.start_system_name.clone() {
                self.player.current_system = self
                    .systems
                    .iter()
                    .find(|system| system.name == system_name)
                    .cloned();
            }
        }
    }
}
