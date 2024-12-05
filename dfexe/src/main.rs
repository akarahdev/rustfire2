use rustfire::headers::PlayerEvent;
use rustfire::items::{Component, Location, Number};
use rustfire::registry;
use rustfire::selections::targets::EventDefault;
use rustfire::selections::{AllOf, Selection};

registry!(main -> {
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    AllOf::player()
        .distance_amount(Location::new_const(10.0, 50.0, 10.0), Number::new("3"))
        .random()
        .send_message(Component::new("Hello there!"));
}
