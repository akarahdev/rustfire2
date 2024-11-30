pub use rustfire::api as df;
use rustfire::api::items::item::Item;
use rustfire::api::selection::EventDefault;
use rustfire::{call, comp, headers, num, start};
use rustfire::api::cf::control::Control;
use rustfire::api::cf::repeat::Repeat;
use rustfire::api::cf::time::Duration;
use rustfire::api::player::Player;

headers! {
    PlayerEvent::join => fn on_join;
    PlayerEvent::respawn => fn on_respawn;

    Function::give_kit => fn give_kit;

    Process::player_loop => fn player_loop;
}

pub fn give_kit() {
    let default = EventDefault::player();
    default.give_item(Item::new("minecraft:mace"));
    default.give_item(Item::new("minecraft:diamond_helmet"));
    default.give_item(Item::new("minecraft:diamond_chestplate"));
    default.give_item(Item::new("minecraft:diamond_leggings"));
    default.give_item(Item::new("minecraft:diamond_boots"));
    default.give_item(Item::new("minecraft:wind_charge").with_count(num!(16)));
}

pub fn on_join(default: EventDefault<Player>) {
    default.send_message(comp!("Hello world!"));
    start!(player_loop);
    call!(give_kit);
}

pub fn on_respawn(default: EventDefault<Player>) {
    call!(give_kit);
}

pub fn player_loop() {
    Repeat::forever(|| {
        EventDefault::player().send_action_bar(comp!("Hi!!"));
        Control::wait(Duration::ticks(1));
    });
}