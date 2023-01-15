use super::{adventureland::ADVENTURELAND, mini_adventure::MINI_ADVENTURE};

const GAME_LIST: [GameEntry; 2] = [
    GameEntry {
        name: "adventureland",
        data: ADVENTURELAND,
    },
    GameEntry {
        name: "mini-adventure",
        data: MINI_ADVENTURE,
    },
];

struct GameEntry {
    name: &'static str,
    data: &'static str,
}

pub fn get_game_data(name: &str) -> Option<String> {
    GAME_LIST
        .into_iter()
        .find(|g| g.name == name.to_lowercase().trim())
        .map(|g| String::from(g.data))
}
