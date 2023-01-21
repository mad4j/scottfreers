use super::{
    adventureland::ADVENTURELAND, adventures_of_buckaroo_banzai::ADVENTURES_OF_BUCKAROO_BANZAI,
    ghost_town::GHOST_TOWN, mini_adventure::MINI_ADVENTURE,
};

const GAME_LIST: [GameEntry; 4] = [
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
    GameEntry {
        name: "adventures-of-buckaroo-banzai",
        data: ADVENTURES_OF_BUCKAROO_BANZAI,
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
