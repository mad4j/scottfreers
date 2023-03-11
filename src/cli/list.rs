
use crate::games::utils::get_game_names;


pub fn cli_list() {

    for name in get_game_names() {
        println!("{}", name);
    }

}