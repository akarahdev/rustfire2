use crate::api::items::item::Item;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::api::items::VarItem;
use crate::api::player::player_action;

player_action! {
    fn launch_proj => "LaunchProj";
    arg proj: Item;
}

player_action! {
    fn set_tick_rate => "SetTickRate";
    arg tick_rate: Number;
}

player_action! {
    fn display_block => "DisplayBlock";
    arg block: Item;
    arg loc: Location;
}

player_action! {
    fn display_block_with_state => "DisplayBlock";
    arg block: Item;
    arg loc: Location;
    arg state: String;
}

player_action! {
    fn clear_display_block => "ClearDispBlock";
    arg pos: Location;
}

player_action! {
    fn clear_display_region => "ClearDispBlock";
    arg start: Location;
    arg end: Location;
}

player_action! {
    fn display_fracture => "DisplayFracture";
    arg pos: Location;
    arg stage: Number;
}

player_action! {
    fn hide_entity => "HideEntity";
    arg uuid: String;
    tag "Ignore Formatting" => "True";
    tag "Set Hidden" => "True";
}