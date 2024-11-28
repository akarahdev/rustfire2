use std::ops::Deref;
use rustfire::api::player::Player;
use rustfire::api::selection::{EventDefault, Selection};
use rustfire::{comp, num, str, subscribe};

subscribe! {
    join for PlayerEvent join;
    left_click for PlayerEvent left_click;
}

fn join(default: EventDefault<Player>) {
    let sel = default
        .filter_random_amount(num!(4))
        .filter_random_amount(num!(16))
        .filter_random_amount(num!(34))
        .filter_random_amount(num!(32))
        .filter_random();
    sel.send_message(comp!("Test"));
    sel.send_message(comp!("Hello world!"));
    sel.saved_data().put(str!("key"), comp!("value").into());
}

fn left_click(default: EventDefault<Player>) {
    default.send_message(comp!("You just clicked, congrats!"));
}
