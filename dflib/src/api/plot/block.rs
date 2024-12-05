use crate::items::{Item, Location, String};
use crate::plot::game_action;

game_action! {
    fn set_block => "SetBlock";

    arg block: Item;
    arg loc: Location;
}

game_action! {
    fn set_region => "SetRegion";

    arg block: Item;
    arg corner1: Location;
    arg corner2: Location;
}

game_action! {
    fn set_block_data => "SetBlockData";

    arg location: Location;
    arg data: String;
}