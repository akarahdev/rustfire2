use rustfire::api::player::Player;
use rustfire::api::selection::EventDefault;
use rustfire::{comp, num, subscribe};
use rustfire::api::items::item::Item;
use rustfire::api::items::list::List;
use rustfire::api::items::number::Number;

subscribe! {
    join for PlayerEvent::join();
    leave for PlayerEvent::leave();
}

fn join(default: EventDefault<Player>) {
    let other = List::new();
    other.append(num!(20));
    let list = List::new();
    list.append(num!(10));
    list.append_list(other)
        .flatten()
        .reversed()
        .randomize();
    default.send_message(comp!("") + list);
}

fn leave(default: EventDefault<Player>) {

}