use std::sync::Arc;

use gdnative::{FromVariant, ToVariant};

use helpers::max;

/// An outfit
#[derive(Debug, Clone)]
pub struct Outfit {
    /// it's name
    pub name: String,
    /// it's mass
    pub mass: i32,
    /// it's category
    pub category: OutfitCategory,
    /// does this outfit has engine properties
    pub engine: Vec<OutfitEngine>,
}

/// An engine part of an outfit
#[derive(Debug, Clone, Copy)]
pub struct OutfitEngine {
    /// type of engine
    pub ty: EngineType,
    /// it's thrust
    pub power: f32,
    /// it's energy consumption
    pub energy_consumption: f32,
    /// it's heat generation
    pub heat_generation: f32,
}

/// Type of an engine
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineType {
    /// forward thrust
    Thrust,
    /// turning power
    Turn,
    /// reverse thrust
    ReverseThrust,
}

/// outfit categories, used for grouping when displaying
#[derive(Debug, Clone, Copy)]
pub enum OutfitCategory {
    /// outfits that improve how a ship works
    Systems,
    /// outfits that help capture or defence
    HandToHand,
    /// outfits that help a ship move
    Engines,
    /// no specific category
    Unspecified,
}

/// A ship
#[derive(Debug, Clone)]
pub struct Ship {
    /// it's name
    pub name: String,
    /// it's sprite
    pub sprite: String,
    /// it's outfits
    pub outfits: Vec<Outfit>,
    /// it's drag
    pub drag: f32,
    /// it's mass
    pub base_mass: u32,
}

impl Ship {
    /// get drag of the ship
    pub fn get_drag(&self) -> f32 {
        self.drag
    }

    /// get mass of the ship plus it's outfits
    pub fn get_mass(&self) -> f32 {
        let mut total_mass: f32 = self.base_mass as f32;
        total_mass += self
            .outfits
            .iter()
            .map(|outfit| outfit.mass as f32)
            .sum::<f32>();
        max!(total_mass, 0.0)
    }

    /// get the sum of the thrust of all engines
    pub fn get_forward_thrust(&self) -> f32 {
        self.outfits
            .iter()
            .flat_map(|outfit| outfit.engine.iter())
            .filter_map(|engine| {
                if engine.ty == EngineType::Thrust {
                    Some(engine.power)
                } else {
                    None
                }
            })
            .sum()
    }

    /// get the sum of the turn of all engines
    pub fn get_turn(&self) -> f32 {
        self.outfits
            .iter()
            .flat_map(|outfit| outfit.engine.iter())
            .filter_map(|engine| {
                if engine.ty == EngineType::Turn {
                    Some(engine.power)
                } else {
                    None
                }
            })
            .sum()
    }
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
#[derive(Debug, ToVariant, FromVariant, Clone)]
pub struct Object {
    /// it's sprite
    pub sprite: Option<String>,
    /// it's distance to it's parent
    pub distance: f32,
    /// it's period
    pub period: f32,
    /// it's subobjects
    pub objects: Vec<Object>,
}

/// A system
#[derive(Debug, Clone)]
pub struct System {
    /// it's name
    pub name: String,
    /// list of stellar object in it
    pub objects: Vec<Object>,
}
