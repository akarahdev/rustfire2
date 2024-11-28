use rustfire::api::done;
use rustfire::api::event::PlayerEvent;
use rustfire::comp;

fn main() {
    PlayerEvent::join(|target| {
        target.send_message(comp!("Hello world!"));
    });
    done();
}
