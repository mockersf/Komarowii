use gdnative::*;

type OwnerNode = Node;

#[derive(NativeClass, Debug)]
#[inherit(OwnerNode)]
#[user_data(gdnative::user_data::MutexData<State>)]
#[register_with(Self::register_properties)]
pub struct State {
    pub game: game_data::Game,
}

unsafe impl Send for State {}

#[methods]
impl State {
    fn _init(_owner: OwnerNode) -> Self {
        let mut es_game_data = game_data::ESGameLoader::empty();

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
            es_game_data.load(&game_data_file.get_as_text().to_string());
            game_data_file.close();
        }
        data_dir.list_dir_end();

        let game_data = es_game_data.create_game().unwrap();
        State { game: game_data }
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder
            .add_property("value")
            .with_default(42)
            .with_setter(State::set_value)
            .with_getter(State::get_value)
            .done();
    }

    #[export]
    fn set_value(&mut self, _owner: OwnerNode, value: i64) {
        godot_print!("setting value to {:?}", value);
    }

    #[export]
    fn get_value(&self, _owner: OwnerNode) -> i64 {
        godot_print!("getting value");
        5
    }
}
