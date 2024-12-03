use crate::api::items::component::Component;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::{player_action, Player};

player_action! {
    fn send_message => "SendMessage",
    arg message: Component,
    tag "Inherit Styles" => "False",
    tag "Text Value Merging" => "Add Spaces",
    tag "Alignment Mode" => "Regular",
}

player_action! {
    fn send_action_bar => "ActionBar",
    arg message: Component,
    tag "Inherit Styles" => "False",
    tag "Text Value Merging" => "Add Spaces",
}

player_action! {
    fn send_title => "SendTitle",
    arg title: Component,
    arg subtitle: Component,
    arg duration: Number,
    arg fade_in: Number,
    arg fade_out: Number,
}