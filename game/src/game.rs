use euclid::{vec2, Angle, Rotation2D, UnknownUnit, Vector2D};
use gdnative::*;
use rand::Rng;

use helpers::{max, min, stringify_fn};

use crate::square_of_background_stars::{self, SquareOfBackgroundStars};
use crate::stellar_object::StellarObject;

const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 5.0;
const CHANGE_ZOOM_SIGNAL: &str = "change_zoom";
const BACKGROUND_PARALLAX_SCALE: f32 = 0.2;

type OwnerNode = Node2D;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
#[register_with(Self::register_signals)]
pub struct Game {
    star_scene: Option<PackedScene>,
    player_scene: Option<PackedScene>,
    background_square_scene: Option<PackedScene>,
    filled_background: std::collections::HashSet<(i64, i64)>,
    player: Player,
    zoom: f32,
    zoom_change: f32,
}

struct Player {
    direction: Angle<f32>,
    speed: Vector2D<f32, UnknownUnit>,
}

unsafe impl Send for Game {}

const MAX_ORIGINAL_SPEED: f32 = 10.0;

#[methods]
impl Game {
    fn register_signals(builder: &init::ClassBuilder<Self>) {
        builder.add_signal(init::Signal {
            name: CHANGE_ZOOM_SIGNAL,
            args: &[init::SignalArgument {
                name: "data",
                default: Variant::from_f64(1.0),
                export_info: init::ExportInfo::new(VariantType::F64),
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
    }

    fn _init(_owner: OwnerNode) -> Self {
        let mut rng = rand::thread_rng();
        let start_speed = vec2(
            rng.gen_range(-MAX_ORIGINAL_SPEED, MAX_ORIGINAL_SPEED),
            rng.gen_range(-MAX_ORIGINAL_SPEED, MAX_ORIGINAL_SPEED),
        );
        let start_direction = start_speed.angle_from_x_axis();
        Game {
            star_scene: helpers::load_scene("res://game/StellarObject.tscn"),
            player_scene: helpers::load_scene("res://game/Player.tscn"),
            background_square_scene: helpers::load_scene("res://game/SquareOfBackgroundStars.tscn"),
            filled_background: std::collections::HashSet::new(),
            player: Player {
                direction: start_direction + Angle::pi(),
                speed: start_speed,
            },
            zoom: 1.0,
            zoom_change: 0.0,
        }
    }

    #[export]
    fn _ready(&mut self, owner: OwnerNode) {
        let state_node = unsafe { owner.get_node("/root/State".into()) };
        let state_instance: Instance<crate::state::State> =
            unsafe { Instance::try_from_unsafe_base(state_node.unwrap()).unwrap() };
        let state = state_instance.into_script();
        state
            .map(|state| {
                let game_data = &state.game;
                let days_since_beginning = game_data.get_nb_days_elapsed_since_beginning() as f32;
                let mut object_parent = unsafe {
                    owner
                        .get_node("objects".into())
                        .expect("objects is present")
                };
                game_data
                    .player
                    .current_system
                    .objects
                    .iter()
                    .for_each(|object| {
                        if let Some(mut new_stellar_object) = self
                            .star_scene
                            .as_ref()
                            .and_then(|star_scene| (&star_scene).instance(0))
                            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
                        {
                            unsafe {
                                if let Some(ref sprite) = object.sprite {
                                    let texture = ResourceLoader::godot_singleton()
                                        .load(
                                            format!("res://images/{}.png", sprite).into(),
                                            "Texture".into(),
                                            false,
                                        )
                                        .and_then(|s| s.cast::<Texture>());
                                    let mut sprite = new_stellar_object
                                        .get_node("Sprite".into())
                                        .unwrap()
                                        .cast::<Sprite>()
                                        .unwrap();
                                    sprite.set_texture(texture);
                                }
                                let rota = Rotation2D::new(Angle::radians(
                                    days_since_beginning / object.period
                                        * 2.0
                                        * std::f32::consts::PI,
                                ));
                                let position = vec2::<f32, UnknownUnit>(0.0, object.distance);
                                let position = rota.transform_vector(position);
                                new_stellar_object.translate(position);
                                new_stellar_object.call_deferred(
                                    stringify_fn!(StellarObject, set_subobjects),
                                    &[
                                        object.objects.to_variant(),
                                        days_since_beginning.to_variant(),
                                    ],
                                );

                                object_parent.add_child(Some(new_stellar_object.to_node()), false);
                            }
                        };
                    });

                let mut ship_parent =
                    unsafe { owner.get_node("ships".into()).expect("ships is present") };
                if let Some(new_player) = self
                    .player_scene
                    .as_ref()
                    .and_then(|player_scene| (&player_scene).instance(0))
                    .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
                {
                    unsafe {
                        let sprite = game_data.player.ship.sprite.clone();
                        let texture = ResourceLoader::godot_singleton()
                            .load(
                                format!("res://images/{}.png", sprite).into(),
                                "Texture".into(),
                                false,
                            )
                            .and_then(|s| s.cast::<Texture>());
                        let mut sprite = new_player
                            .get_node("Sprite".into())
                            .unwrap()
                            .cast::<Sprite>()
                            .unwrap();
                        sprite.set_texture(texture);
                        sprite.set_scale(vec2(0.5, 0.5));
                        let mut node = new_player.to_node();
                        node.set_name("player".into());
                        ship_parent.add_child(Some(node), false);
                    }
                }
                ()
            })
            .unwrap();
    }

    #[export]
    fn _process(&mut self, owner: OwnerNode, delta: f32) {
        self.update_background(owner);

        self.player_movement(owner, delta);

        self.zoom(owner);
    }

    fn update_background(&mut self, mut owner: OwnerNode) {
        let view = unsafe { owner.get_viewport().unwrap().get_visible_rect() };
        let mut background_parent = unsafe {
            owner
                .get_node("ParallaxBackground/ParallaxLayer/background".into())
                .expect("node background is present")
        };
        let player_position = unsafe {
            owner
                .get_node("ships/player".into())
                .and_then(|new_node| new_node.cast::<Node2D>())
                .unwrap()
                .get_position()
        };
        let min_x =
            player_position.x * BACKGROUND_PARALLAX_SCALE - view.size.width * self.zoom / 2.0;
        let max_x =
            player_position.x * BACKGROUND_PARALLAX_SCALE + view.size.width * self.zoom / 2.0;
        let min_y =
            player_position.y * BACKGROUND_PARALLAX_SCALE - view.size.height * self.zoom / 2.0;
        let max_y =
            player_position.y * BACKGROUND_PARALLAX_SCALE + view.size.height * self.zoom / 2.0;
        for background_x in min_x as i64 / square_of_background_stars::SQUARE_SIZE - 1
            ..=max_x as i64 / square_of_background_stars::SQUARE_SIZE + 1
        {
            for background_y in min_y as i64 / square_of_background_stars::SQUARE_SIZE - 1
                ..=max_y as i64 / square_of_background_stars::SQUARE_SIZE + 1
            {
                if !self
                    .filled_background
                    .contains(&(background_x, background_y))
                {
                    if let Some(mut background_square) = self
                        .background_square_scene
                        .as_ref()
                        .and_then(|bs_scene| (&bs_scene).instance(0))
                        .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
                    {
                        unsafe {
                            background_square.translate(vec2(
                                (background_x * square_of_background_stars::SQUARE_SIZE) as f32,
                                (background_y * square_of_background_stars::SQUARE_SIZE) as f32,
                            ));
                            background_square.call_deferred(
                                stringify_fn!(SquareOfBackgroundStars, change_zoom),
                                &[Variant::from_f64(self.zoom.into())],
                            );
                            background_parent.add_child(Some(background_square.to_node()), false);
                            owner
                                .connect(
                                    GodotString::from_str(CHANGE_ZOOM_SIGNAL),
                                    Some(background_square.to_object()),
                                    stringify_fn!(SquareOfBackgroundStars, change_zoom),
                                    VariantArray::new(),
                                    0,
                                )
                                .unwrap();
                        }
                    }
                    self.filled_background.insert((background_x, background_y));
                }
            }
        }
    }

    fn player_movement(&mut self, owner: OwnerNode, delta: f32) {
        let thrust = 11.5;
        let mass = 182.0;
        let drag = 1.7;
        let turn = 307.0;

        let es_thrust_ratio = 3600.0;
        let es_turn_ratio = 60.0;
        let max_velocity_ratio = 0.005;
        let turn_ratio = 0.01;

        let acceleration = thrust * es_thrust_ratio / mass;
        let max_velocity = thrust * es_thrust_ratio / drag * max_velocity_ratio;

        let angular_speed = Angle::degrees(turn * es_turn_ratio * turn_ratio);
        let mut new_acceleration_vec: Vector2D<f32, UnknownUnit> = vec2(0.0, 0.0);
        let mut rotation = self.player.direction;
        let input = Input::godot_singleton();
        if input.is_action_pressed("ui_right".into()) {
            rotation += angular_speed * delta;
        }
        if input.is_action_pressed("ui_left".into()) {
            rotation -= angular_speed * delta;
        }
        if input.is_action_pressed("ui_down".into()) {
            let current_mov_angle = self.player.speed.angle_from_x_axis() + Angle::pi();
            let target = current_mov_angle + Angle::pi();
            let turn = target - rotation;
            rotation += turn.signed() * delta;
        }
        if input.is_action_pressed("ui_up".into()) {
            new_acceleration_vec.x -= acceleration;
        }
        self.player.direction = rotation.positive();
        let rota = Rotation2D::new(rotation);
        let acceleration = rota.transform_vector(new_acceleration_vec);
        let mut player = unsafe { owner.get_node("ships/player".into()) }
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            .unwrap();
        let new_speed = acceleration * delta + self.player.speed;
        self.player.speed = new_speed.with_max_length(max_velocity);
        unsafe {
            player.set_rotation((rotation - Angle::frac_pi_2()).get() as f64);
            player.set_position(player.get_position() + self.player.speed * delta);
        }
    }

    fn zoom(&mut self, mut owner: OwnerNode) {
        let input = Input::godot_singleton();
        let player = unsafe { owner.get_node("ships/player".into()) }
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            .unwrap();

        if input.is_key_pressed(GlobalConstants::KEY_PAGEDOWN)
            || input.is_mouse_button_pressed(GlobalConstants::BUTTON_WHEEL_UP)
        {
            self.zoom_change = 0.1;
        }
        if input.is_key_pressed(GlobalConstants::KEY_ESCAPE) {
            unsafe {
                owner
                    .get_tree()
                    .expect("was able to get tree from node")
                    .change_scene("res://menu/MainMenu.tscn".into())
                    .expect("was able to change scene");
            }
        }
        if input.is_key_pressed(GlobalConstants::KEY_PAGEUP)
            || input.is_mouse_button_pressed(GlobalConstants::BUTTON_WHEEL_DOWN)
        {
            self.zoom_change = -0.1;
        }
        if self.zoom_change != 0. {
            self.zoom = max!(min!(self.zoom + self.zoom_change, ZOOM_MAX), ZOOM_MIN);
            let mut camera = unsafe { player.get_node("Camera2D".into()) }
                .and_then(|new_node| unsafe { new_node.cast::<Camera2D>() })
                .unwrap();
            unsafe {
                camera.set_zoom(vec2(self.zoom, self.zoom));
            }
            self.zoom_change = 0.;
            unsafe {
                owner.emit_signal(
                    GodotString::from_str(CHANGE_ZOOM_SIGNAL),
                    &[Variant::from_f64(self.zoom.into())],
                );
            }
        }
    }

    #[export]
    fn _input(&mut self, _owner: OwnerNode, event: InputEvent) {
        if let Some(iepg) = event.cast::<InputEventPanGesture>() {
            let delta = iepg.get_delta();
            if delta.x.abs() < 0.04 {
                self.zoom_change = delta.y / 5.;
            }
        }
    }
}
