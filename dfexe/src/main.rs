mod optional;
mod safelist;

use crate::safelist::SafeList;
use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::selections::targets::EventDefault;
use rustfire::{comp, num, registry};

registry!({
    PlayerEvent::Join.declare(on_join);
    PlayerEvent::Leave.declare(on_join);
    PlayerEvent::LeftClick.declare(on_join);
    PlayerEvent::RightClick.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));
    EventDefault::player().open_inv();

    let list = SafeList::new();
    list.append(num!(10));
    list.append(num!(20));
    list.append(num!(30));

    EventDefault::player().send_message(comp!("Value of list[6] is:") + list.get(num!(6)));

    EventDefault::player().send_message(comp!("Value of list[2] is:") + list.get(num!(2)));

    EventDefault::player().send_message(comp!("Value of list[19] is:") + list.get(num!(19)));
}
