use gdnative::*;

use helpers::stringify_fn;

type OwnerNode = Area2D;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct StellarObject {
    star_scene: Option<PackedScene>,
}

unsafe impl Send for StellarObject {}

#[methods]
impl StellarObject {
    fn _init(_owner: OwnerNode) -> Self {
        StellarObject {
            star_scene: helpers::load_scene("res://game/StellarObject.tscn"),
        }
    }

    #[export]
    fn _ready(&mut self, mut owner: OwnerNode) {
        unsafe {
            let target = owner;
            owner
                .connect(
                    helpers::Signal::AreaEntered.into(),
                    Some(target.to_object()),
                    stringify_fn!(Self, _entered_stellar_object),
                    VariantArray::new(),
                    0,
                )
                .expect("signal connected");
        }
    }

    #[export]
    pub fn set_subobjects(
        &self,
        owner: OwnerNode,
        objects: Vec<game_data::Object>,
        days_since_beginning: f32,
    ) {
        let mut object_parent = unsafe {
            owner
                .get_node("objects".into())
                .expect("objects is present")
        };
        objects.iter().for_each(|object| {
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
                    let rota = euclid::Rotation2D::new(euclid::Angle::radians(
                        days_since_beginning / object.period * 2.0 * std::f32::consts::PI,
                    ));
                    let position = euclid::vec2::<f32, euclid::UnknownUnit>(0.0, object.distance);
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
    }

    #[export]
    fn _entered_stellar_object(&mut self, _owner: OwnerNode, _entered: Area2D) {
        godot_print!("over stellar object");
    }
}
