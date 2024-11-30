use rustfire::{comp, headers, num, start};
use rustfire::api::event::PlayerEvent;
use rustfire::api::flow::{Control, Duration, Repeat};
use rustfire::api::player::Player;
use rustfire::api::selection::EventDefault;

headers! {
    PlayerEvent::join => fn on_join;
    Process::player_loop => fn player_loop;
}

pub fn on_join(default: EventDefault<Player>) {
    default.send_message(comp!("Hello world!"));
    start!(player_loop);
}

pub fn player_loop() {
    Repeat::forever(|| {
        EventDefault::player().send_message(comp!("Repeating!!"));
        Control::wait(Duration::ticks(1));    
    });
}