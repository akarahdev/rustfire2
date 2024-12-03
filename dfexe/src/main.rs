
use rustfire::{comp, registry};
use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::selections::targets::EventDefault;

registry!({
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));
}