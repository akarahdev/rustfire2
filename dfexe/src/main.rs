use rustfire::api::done;
use rustfire::api::event::PlayerEvent;
use rustfire::api::items::component::Component;
use rustfire::api::items::number::Number;
use rustfire::api::player::Player;

fn main() {
    PlayerEvent::join(|| {
        Player::send_message(Component::new("Hello world!"));
        Player::send_message(Component::cast(Number::new("1") + Number::new("2")));
    });
    done();
}
