use rustfire::headers::PlayerEvent;
use rustfire::items::Component;
use rustfire::registry;
use rustfire::selections::targets::EventDefault;

registry!(main -> {
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(Component::new("Hello world!"));
}