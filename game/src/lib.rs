//! this crate is to handle the main menu of the game

//allowing this lint for errors from gdnative macros
#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::transmute_ptr_to_ptr)]
#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

use gdnative::*;

mod game;
mod square_of_background_stars;
mod state;
mod stellar_object;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<stellar_object::StellarObject>();
    handle.add_class::<square_of_background_stars::SquareOfBackgroundStars>();
    handle.add_class::<state::State>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
