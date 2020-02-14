//! Types that will represent the game state, how to create it from ES data files, how to save/load it, ...

#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    missing_docs
)]

use std::sync::Arc;

mod loader;
pub use loader::ESGameLoader;

/// A ship
#[derive(Debug)]
pub struct Ship {
    /// it's name
    pub name: String,
    /// it's sprite
    pub sprite: String,
}

/// A player
#[derive(Debug)]
pub struct Player {
    /// it's ship
    pub ship: Arc<Ship>,
    /// it's current system
    pub current_system: Arc<System>,
}

/// A stellar object
#[derive(Debug)]
pub struct Object {
    /// it's sprite
    pub sprite: Option<String>,
    /// it's distance to it's parent
    pub distance: f32,
    /// it's period
    pub period: f32,
}

/// A system
#[derive(Debug)]
pub struct System {
    /// it's name
    pub name: String,
    /// list of stellar object in it
    pub objects: Vec<Object>,
}

/// A Game
#[derive(Debug)]
pub struct Game {
    /// the current date, once found from es data start point
    pub current_date: chrono::NaiveDate,
    /// the player
    pub player: Player,
    /// the list of systems
    pub systems: Vec<Arc<System>>,
    /// the list of ships
    pub ships: Vec<Arc<Ship>>,
}

#[allow(clippy::new_without_default)]
impl Game {
    /// Compute the number of days since the beginning of time (1/1/2020)
    pub fn get_nb_days_elapsed_since_beginning(&self) -> i64 {
        self.get_nb_days_elapsed_since(chrono::NaiveDate::from_ymd(2020, 1, 1))
    }

    /// Compute the number of days since a given date
    pub fn get_nb_days_elapsed_since(&self, start_date: chrono::NaiveDate) -> i64 {
        (self.current_date - start_date).num_days()
    }
}
