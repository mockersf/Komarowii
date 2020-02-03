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
    fn _ready(&mut self, mut owner: OwnerNode) {
        if let Some(new_stellar_object) = self
            .star_scene
            .as_ref()
            .and_then(|star_scene| (&star_scene).instance(0))
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
        {
            unsafe {
                owner.add_child(Some(new_stellar_object.to_node()), false);
            }
        }
        if let Some(new_player) = self
            .player_scene
            .as_ref()
            .and_then(|player_scene| (&player_scene).instance(0))
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
        {
            unsafe {
                let mut node = new_player.to_node();
                node.set_name("player".into());
                owner.add_child(Some(node), false);
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
        let mut player = unsafe { owner.get_node("player".into()) }
            .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            .unwrap();
        unsafe {
            player.set_rotation(rotation as f64 - std::f64::consts::PI / 2.0);
            player.set_position(player.get_position() + movement * speed * delta);
        }
    }
}
