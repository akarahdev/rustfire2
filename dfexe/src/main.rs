use std::ops::Deref;
use rustfire::api::player::Player;
use rustfire::api::selection::{EventDefault, Selection};
use rustfire::{comp, num, str, subscribe};
use rustfire::api::items::boolean::Boolean;
use rustfire::api::items::item::Item;

subscribe! {
    join for PlayerEvent join;
    respawn for PlayerEvent respawn;
    right_click for PlayerEvent right_click;
}

fn join(default: EventDefault<Player>) {
    default.give_item(Item::new("diamond"));
}

fn respawn(default: EventDefault<Player>) {
    default.give_item(Item::new("diamond"));
}

fn right_click(default: EventDefault<Player>) {
    let is_holding = default.is_holding(Item::new("diamond"));
    is_holding.if_true(|| {
        default.launch_proj(Item::new("minecraft:fire_charge"));
    }).or_else(|| {
        default.send_message(comp!("Hold a diamond to shoot a fireball!"))
    });
}

