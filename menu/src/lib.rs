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

mod menu;
mod star;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<star::Star>();
    handle.add_class::<menu::Menu>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
