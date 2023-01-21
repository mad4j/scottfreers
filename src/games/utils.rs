use super::{adventureland::ADVENTURELAND, mini_adventure::MINI_ADVENTURE, ghost_town::GHOST_TOWN};

const GAME_LIST: [GameEntry; 3] = [
    GameEntry {
        name: "adventureland",
        data: ADVENTURELAND,
    },
    GameEntry {
        name: "mini-adventure",
        data: MINI_ADVENTURE,
    },
    GameEntry {
        name: "ghost-town",
        data: GHOST_TOWN,
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
