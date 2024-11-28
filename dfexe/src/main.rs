use std::ops::Deref;
use rustfire::api::player::Player;
use rustfire::api::selection::{EventDefault, Selection};
use rustfire::{comp, num, str, subscribe};
use rustfire::api::items::item::Item;

subscribe! {
    join for PlayerEvent join;
    right_click for PlayerEvent right_click;
}

fn join(default: EventDefault<Player>) {
    default.give_item(Item::new("diamond"));
}

fn right_click(default: EventDefault<Player>) {
    default.is_holding(Item::new("diamond"), || {
        default.launch_proj(Item::new("minecraft:fire_charge"));
    });
}

