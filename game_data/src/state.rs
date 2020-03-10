use gdnative::*;

// use std::sync::{Arc, RwLock};

type OwnerNode = Node;

/// State of a game
#[derive(NativeClass, Debug)]
#[inherit(OwnerNode)]
#[user_data(gdnative::user_data::MutexData<State>)]
#[register_with(Self::register_properties)]
pub struct State {
    /// default state, as parsed from data files
    pub game_data: crate::ESGameLoader,
    /// current state, with actions made by the player
    pub current_game: Option<crate::Game>,
}

unsafe impl Send for State {}

#[methods]
impl State {
    fn _init(_owner: OwnerNode) -> Self {
        let mut es_game_data = crate::loader::UnresolvedESGameLoader::empty();

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

        State {
            game_data: es_game_data.resolve(),
            current_game: None,
        }
    }

    /// return the current game, or a new one if there is not one already
    pub fn current_game_or_new(&mut self) -> &crate::Game {
        if self.current_game.is_none() {
            self.new_game();
        }
        self.current_game.as_ref().unwrap()
    }

    /// create a new game from default state
    pub fn new_game(&mut self) {
        self.current_game = Some(self.game_data.create_game().unwrap())
    }
}
