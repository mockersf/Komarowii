use gdnative::*;

type OwnerNode = Node2D;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct Game {
    star_scene: Option<PackedScene>,
    player_scene: Option<PackedScene>,
    player: Player,
}

struct Player {
    direction: f32,
}

unsafe impl Send for Game {}

#[methods]
impl Game {
    fn _init(_owner: OwnerNode) -> Self {
        Game {
            star_scene: helpers::load_scene("res://game/StellarObject.tscn"),
            player_scene: helpers::load_scene("res://game/Player.tscn"),
            player: Player { direction: 0.0 },
        }
    }

    #[export]
    fn _ready(&mut self, owner: OwnerNode) {
        let mut game_data = game_data::Game::new();

        let mut data_dir = gdnative::Directory::new();
        data_dir.open("res://data".into()).unwrap();

        data_dir.list_dir_begin(false, false).unwrap();
        loop {
            let path = data_dir.get_next();
            if path.is_empty() {
                break;
            }
            if !path.ends_with(&".txt".into()) {
                continue;
            }
            let full_path = format!("res://data/{}", path.to_string());
            let mut game_data_file = gdnative::File::new();
            game_data_file.open(full_path.into(), 1).unwrap();
            game_data.add_data_file(&game_data_file.get_as_text().to_string());
            game_data_file.close();
        }
        data_dir.list_dir_end();

        let days_since_beginning = game_data.get_nb_days_elapsed_since_beginning() as f32;
        let mut object_parent = unsafe {
            owner
                .get_node("objects".into())
                .expect("objects is present")
        };
        game_data
            .player
            .current_system
            .unwrap()
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
                            new_stellar_object
                                .get_node("Sprite".into())
                                .unwrap()
                                .cast::<Sprite>()
                                .unwrap()
                                .set_texture(texture);
                        }
                        godot_print!(
                            "rotation status: period {}, days elapsed {}, angle {}",
                            object.period,
                            days_since_beginning,
                            days_since_beginning / object.period * 2.0 * std::f32::consts::PI
                        );
                        let rota = euclid::Rotation2D::new(euclid::Angle::radians(
                            days_since_beginning / object.period * 2.0 * std::f32::consts::PI,
                        ));
                        let position =
                            euclid::vec2::<f32, euclid::UnknownUnit>(0.0, object.distance);
                        let position = rota.transform_vector(position);
                        godot_print!("---> position {:?}", position);
                        new_stellar_object.translate(position);
                        object_parent.add_child(Some(new_stellar_object.to_node()), false);
                    }
                };
            });

        let mut ship_parent = unsafe { owner.get_node("ships".into()).expect("ships is present") };
        if let Some(new_player) = self
            .player_scene
            .as_ref()
            .and_then(|player_scene| (&player_scene).instance(0))
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
        {
            godot_print!("adding player");
            unsafe {
                if let Some(ref sprite) = game_data.player.ship.map(|s| s.sprite.clone()) {
                    let texture = ResourceLoader::godot_singleton()
                        .load(
                            format!("res://images/{}.png", sprite).into(),
                            "Texture".into(),
                            false,
                        )
                        .and_then(|s| s.cast::<Texture>());
                    new_player
                        .get_node("Sprite".into())
                        .unwrap()
                        .cast::<Sprite>()
                        .unwrap()
                        .set_texture(texture);
                }
                let mut node = new_player.to_node();
                node.set_name("player".into());
                ship_parent.add_child(Some(node), false);
            }
        }
    }

    #[export]
    fn _process(&mut self, owner: OwnerNode, delta: f32) {
        let speed = 100.0;
        let angular_speed = 0.05;
        let mut movement: euclid::Vector2D<f32, euclid::UnknownUnit> = euclid::vec2(0.0, 0.0);
        let mut rotation = self.player.direction;
        let input = Input::godot_singleton();
        if input.is_action_pressed("ui_right".into()) {
            rotation += angular_speed;
        }
        if input.is_action_pressed("ui_left".into()) {
            rotation -= angular_speed;
        }
        if input.is_action_pressed("ui_down".into()) {
            movement.x += 1.0;
        }
        if input.is_action_pressed("ui_up".into()) {
            movement.x -= 1.0;
        }
        self.player.direction = rotation;
        let rota = euclid::Rotation2D::new(euclid::Angle::radians(rotation));
        let movement = rota.transform_vector(movement);
        let mut player = unsafe { owner.get_node("ships/player".into()) }
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            .unwrap();
        unsafe {
            player.set_rotation(rotation as f64 - std::f64::consts::PI / 2.0);
            player.set_position(player.get_position() + movement * speed * delta);
        }
    }
}
