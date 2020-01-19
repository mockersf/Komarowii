//! handle the background stars in the main menu: flicker and death

use gdnative::*;
use rand::Rng;

/// probability a star will die
const DEATH_PROBABILITY: f32 = 0.00001;
/// probability a star will change brightness
const CHANGE_PROBABILITY: f32 = 0.05;
/// how much should we change star's color's alpha
const ALPHA_CHANGE: f32 = 0.05;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Star {
    rng: rand::rngs::ThreadRng,
}

unsafe impl Send for Star {}

#[methods]
impl Star {
    fn _init(_owner: Node2D) -> Self {
        Star {
            rng: rand::thread_rng(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: Node2D) {
        unsafe {
            if let Some(mut visi) = owner.get_node("Star/VisibilityNotifier2D".into()) {
                visi.connect(
                    crate::Signal::ScreenExited.into(),
                    Some(owner.to_object()),
                    "_on_visibility_screen_exited".into(),
                    VariantArray::new(),
                    0,
                )
                .expect("signal connected");
            }
        }
    }

    #[export]
    fn _process(&mut self, mut owner: Node2D, _delta: f32) {
        let proba = self.rng.gen_range(0.0, 1.0);
        // should this star die
        if proba < DEATH_PROBABILITY {
            unsafe {
                owner.queue_free();
            }
        }
        // should this star change brightness
        if proba < CHANGE_PROBABILITY {
            unsafe {
                if let Some(mut star) = owner
                    .get_node("Star".into())
                    .and_then(|node| node.cast::<ColorRect>())
                {
                    let mut color = star.get_frame_color();
                    color.a = if self.rng.gen() {
                        min!(color.a + ALPHA_CHANGE, 0.7)
                    } else {
                        max!(color.a - ALPHA_CHANGE, 0.2)
                    };
                    star.set_frame_color(color)
                }
            }
        }
    }

    #[export]
    fn _on_visibility_screen_exited(&mut self, mut owner: Node2D) {
        unsafe {
            owner.queue_free();
        }
    }
}
