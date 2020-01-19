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

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

mod star;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<star::Star>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

#[cfg(test)]
mod test {
    #[test]
    fn macro_max() {
        assert_eq!(max!(3.3, 5.2), 5.2);
        assert_eq!(max!(5.7, 5.2), 5.7);
    }

    #[test]
    fn macro_min() {
        assert_eq!(min!(3.3, 5.2), 3.3);
        assert_eq!(min!(5.7, 5.2), 5.2);
    }
}
