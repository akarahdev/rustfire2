
use rustfire::{comp, num, registry};
use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::items::item::Item;
use rustfire::api::selections::targets::EventDefault;

registry!({
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));
    EventDefault::player().open_inv();

    for _ in 1..1000 {
        EventDefault::player().set_menu_item(
            Item::new("minecraft:mace")
                .with_count(num!(17)),
            num!(14)
        );
    }
}