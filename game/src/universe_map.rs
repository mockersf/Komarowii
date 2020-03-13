use gdnative::*;

type OwnerNode = Node;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct UniverseMap {
    system_scene: Option<PackedScene>,
    systems: Option<Vec<std::sync::Arc<game_data::data::System>>>,
}

unsafe impl Send for UniverseMap {}

#[methods]
impl UniverseMap {
    fn _init(_owner: OwnerNode) -> Self {
        UniverseMap {
            system_scene: helpers::load_scene("res://game/universe_map/system.tscn"),
            systems: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: OwnerNode) {
        self.set_map_visibility(owner, false, euclid::vec2(0.0, 0.0));
    }

    pub fn add_systems(&mut self, systems: &Vec<std::sync::Arc<game_data::data::System>>) {
        self.systems = Some(systems.iter().cloned().collect());
    }

    #[export]
    pub fn set_map_visibility(
        &mut self,
        owner: OwnerNode,
        visibility: bool,
        center: euclid::Vector2D<f32, euclid::UnknownUnit>,
    ) {
        unsafe {
            owner
                .get_node("map/background".into())
                .unwrap()
                .cast::<ColorRect>()
                .unwrap()
                .set_visible(visibility);

            let mut systems = owner
                .get_node("map/systems".into())
                .unwrap()
                .cast::<Node2D>()
                .unwrap();
            systems.set_visible(visibility);

            systems.set_position(-center);
            let view = owner.get_viewport().unwrap().get_visible_rect();
            systems.translate(view.size.to_vector() / 2.0);
        }
    }

    #[export]
    fn _process(&mut self, owner: OwnerNode, _delta: f32) {
        let mut parent = unsafe { owner.get_node("map/systems".into()).unwrap() };
        if unsafe { parent.get_child_count() } == 0 {
            if let Some(systems) = &self.systems {
                systems.iter().for_each(|system| {
                    if let Some(mut new_system) = self
                        .system_scene
                        .as_ref()
                        .and_then(|system_scene| (&system_scene).instance(0))
                        .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
                    {
                        unsafe {
                            new_system.translate(system.position);

                            parent.add_child(Some(new_system.to_node()), false);
                        }
                    };
                })
            }
        }
    }
}
