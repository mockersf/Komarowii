use euclid::vec2;
use gdnative::*;
use rand::Rng;

/// probability there will be a star at any point
const STAR_PROBABILITY: f32 = 0.0003;

pub const SQUARE_SIZE: i64 = 500;

const STAR_COLORS: [(f32, Color); 4] = [
    (
        0.9,
        Color {
            r: 0.6,
            g: 0.6,
            b: 1.0,
            a: 0.6,
        },
    ),
    (
        0.8,
        Color {
            r: 1.0,
            g: 1.0,
            b: 0.6,
            a: 0.6,
        },
    ),
    (
        0.7,
        Color {
            r: 1.0,
            g: 0.6,
            b: 0.6,
            a: 0.6,
        },
    ),
    (
        0.6,
        Color {
            r: 0.7,
            g: 0.7,
            b: 0.7,
            a: 0.6,
        },
    ),
];

type OwnerNode = Node2D;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct SquareOfBackgroundStars {
    rng: rand::rngs::ThreadRng,
    star_scene: Option<PackedScene>,
}

unsafe impl Send for SquareOfBackgroundStars {}

#[methods]
impl SquareOfBackgroundStars {
    fn _init(_owner: OwnerNode) -> Self {
        SquareOfBackgroundStars {
            rng: rand::thread_rng(),
            star_scene: helpers::load_scene("res://game/BackgroundStar.tscn"),
        }
    }

    #[export]
    fn _ready(&mut self, mut owner: OwnerNode) {
        let mut star_count = 0;
        let target_star_count: i64 =
            (SQUARE_SIZE as f32 * SQUARE_SIZE as f32 * STAR_PROBABILITY) as i64;
        while star_count < target_star_count {
            if let Some(mut new_star) = self
                .star_scene
                .as_ref()
                .and_then(|star_scene| (&star_scene).instance(0))
                .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            {
                let x = self.rng.gen_range(0.0, SQUARE_SIZE as f32);
                let y = self.rng.gen_range(0.0, SQUARE_SIZE as f32);
                let color_sel = self.rng.gen_range(0.0, 1.0);
                let color = STAR_COLORS
                    .iter()
                    .filter(|(p, _)| p < &color_sel)
                    .map(|(_, color)| color)
                    .next()
                    .unwrap_or(&STAR_COLORS[STAR_COLORS.len() - 1].1);
                unsafe {
                    new_star.translate(vec2(x, y));
                    new_star
                        .get_node("Star".into())
                        .and_then(|node| node.cast::<ColorRect>())
                        .expect("ColorRect Star is present in a star")
                        .set_frame_color(color.clone());
                    owner.add_child(Some(new_star.to_node()), false);
                }
            }
            star_count += 1;
        }
    }

    #[export]
    pub fn change_zoom(&self, owner: OwnerNode, zoom: f64) {
        unsafe {
            owner
                .get_children()
                .iter_mut()
                .filter_map(|c| c.try_to_object::<Node2D>())
                .filter_map(|node| node.get_node("Star".into()))
                .filter_map(|node| node.cast::<ColorRect>())
                .for_each(|mut star_node| star_node.set_size(vec2(zoom as f32, zoom as f32)));
        }
    }
}
