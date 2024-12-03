use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::{player_action, Player};
use crate::api::push_block;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::{BlockType, TemplateBlock};
use crate::codetemplate::args::Item as DFItem;

player_action! {
    fn teleport => "Teleport";
    arg location: Location;
    tag "Keep Current Rotation" => "False";
    tag "Keep Velocity" => "False";
}

player_action! {
    fn launch_up => "LaunchUp";
    arg amount: Number;
    tag "Add to Current Velocity" => "True";
}

player_action! {
    fn launch_fwd => "LaunchFwd";
    arg amount: Number;
    tag "Add to Current Velocity" => "True";
    tag "Launch Axis" => "Yaw Only";
}

player_action! {
    fn launch_towards => "LaunchToward";
    arg pos: Location;
    tag "Add to Current Velocity" => "True";
    tag "Ignore Distance" => "False";
}