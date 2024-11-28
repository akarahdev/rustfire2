use rustfire::api::done;
use rustfire::api::event::PlayerEvent;
use rustfire::api::items::component::Component;
use rustfire::api::items::number::Number;
use rustfire::api::player::Player;
use rustfire::{comp, num};

fn main() {
    PlayerEvent::join(|| {
        Player::send_message(comp!("Hello world!"));
        Player::send_message(
                comp!("Ok! The solution is: ") +
                    (num!("1.23") / num!("2.53"))
        );
    });
    done();
}
