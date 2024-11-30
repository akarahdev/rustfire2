use rustfire::api::items::item::Item;
use rustfire::api::selections::targets::EventDefault;
use rustfire::{call, comp, num, registry, start};
use rustfire::api::cf::control::Control;
use rustfire::api::cf::repeat::Repeat;
use rustfire::api::cf::time::Duration;
use rustfire::api::done;
use rustfire::api::entity::Entity;
use rustfire::api::headers::functions::Functions;
use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::headers::processes::Processes;
use rustfire::api::items::loc::Location;
use rustfire::api::items::number::Number;

registry!({
    PlayerEvent::Join.declare(on_join);
    PlayerEvent::Respawn.declare(on_respawn);

    Processes::declare("player_loop", player_loop);

    Functions::declare("give_kit", give_kit);

    done();
});

pub fn give_kit() {
    EventDefault::player().give_items(&[
        Item::new("minecraft:mace"),
        Item::new("minecraft:wind_charge")
            .with_count(num!(16)),
        Item::new("minecraft:diamond_helmet"),
        Item::new("minecraft:diamond_chestplate"),
        Item::new("minecraft:diamond_leggings"),
        Item::new("minecraft:diamond_boots")
    ]);

    EventDefault::player().send_message(comp!("Matrix is: ") + sum.as_list());
}

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));
    start!(player_loop);
    call!(give_kit);
    EventDefault::player().save_inventory();
}

pub fn on_respawn() {
    EventDefault::player().load_inventory();
}

pub fn player_loop() {
    Repeat::forever(|| {
        EventDefault::player().send_action_bar(comp!("Hi!!"));
        Control::wait(Duration::ticks(1));
    });
}

pub fn on_dmg(entity: EventDefault<Entity>) {
    entity.teleport(Location::new_const(25.5, 50.5, 25.5));
}