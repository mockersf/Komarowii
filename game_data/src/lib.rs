//! Types that will represent the game state, how to create it from ES data files, how to save/load it, ...

#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    missing_docs
)]

use std::sync::Arc;

mod loader;
pub use loader::ESGameLoader;
mod state;
pub use state::State;
/// data types that represent a game
pub mod data;
mod unresolved_data;

/// A Game
#[derive(Debug)]
pub struct Game {
    /// the current date, once found from es data start point
    pub current_date: chrono::NaiveDate,
    /// the player
    pub player: data::Player,
    /// the list of systems
    pub systems: Vec<Arc<data::System>>,
    /// the list of ships
    pub ships: Vec<Arc<data::Ship>>,
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
