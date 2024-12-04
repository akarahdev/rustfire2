use crate::api::config::PlotRank;
use crate::api::items::component::Component;
use crate::api::items::item::Item;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::player_action;

player_action! {
    requires PlotRank::Emperor;
    fn open_inv => "OpenInv";
}

player_action! {
    requires PlotRank::Emperor;
    fn set_menu_item => "SetMenuItem";

    arg item: Item;
    arg slot: Number;
}

player_action! {
    requires PlotRank::Emperor;
    fn set_inv_name => "SetInvName";

    arg name: Component;
}

player_action! {
    requires PlotRank::Emperor;
    fn close_inv => "CloseInv";
}

player_action! {
    requires PlotRank::Emperor;
    fn open_container => "OpenBlockInv";

    arg container: Location;
}
