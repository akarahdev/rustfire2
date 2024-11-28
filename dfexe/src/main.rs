use rustfire::api::done;
use rustfire::api::event::PlayerEvent;
use rustfire::api::items::component::Component;
use rustfire::api::items::number::Number;
use rustfire::api::player::Player;
use rustfire::{comp, num, str};
use rustfire::api::items::dict::Dictionary;
use rustfire::api::items::list::List;
use rustfire::api::items::string::String;

fn main() {
    PlayerEvent::join(|| {
        Player::send_message(comp!("Hello world!"));
        Player::send_message(
                comp!("Ok! The solution is: ") +
                    (num!(1.23) / num!(2.53))
        );

        let list = List::new();
        list.append(num!(10));
        list.append(num!(20));
        list.append(num!(30));

        Player::send_message(comp!("") + list);

        let dict = Dictionary::new();
        dict.put(str!("Endistic"), num!(10));
        dict.put(str!("IcyBlizzardPengu"), num!(-45));
        dict.put(str!("IHaveAqInMyName"), num!(45.381));

        Player::send_message(comp!("") + dict);
    });
    done();
}
