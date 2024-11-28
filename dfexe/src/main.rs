use rustfire::api::done;
use rustfire::api::event::PlayerEvent;
use rustfire::api::items::component::Component;
use rustfire::api::items::number::Number;
use rustfire::api::player::Player;
use rustfire::{comp, dict, list, num, str};
use rustfire::api::items::dict::Dictionary;
use rustfire::api::items::list::List;
use rustfire::api::items::loc::Location;
use rustfire::api::items::string::String;

fn main() {
    PlayerEvent::join(|| {
        Player::send_message(comp!("Hello world!"));
        Player::send_message(
                comp!("Ok! The solution is: ") +
                    (num!(1.23) / num!(2.53))
        );

        let list = list![num!(10), num!(20), num!(30)];

        Player::send_message(comp!("") + list);

        let dict = dict! {
            str!("Endistic") => num!(10),
            str!("IcyBlizzardPengu") => num!(-45),
            str!("IHaveAqInMyName") => num!(45.381)
        };
        Player::send_message(comp!("") + dict);

        Player::teleport(Location::new_const(0.0, 100.0, 0.0));
    });
    done();
}
