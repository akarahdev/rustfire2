use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::selections::targets::EventDefault;
use rustfire::{comp, num, registry};
use rustfire::api::cf::control::Control;
use rustfire::api::cf::repeat::Repeat;
use rustfire::api::items::list::List;
use rustfire::std::iter::Iterator;

registry!({
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));
    
    let list = List::new();
    list.append(num!(15));
    list.append(num!(45));
    list.append(num!(27));
    
    let iterator = list.iter();
    
    Repeat::forever(|| {
        let next = iterator.next();
        next.if_none(|| {
            Control::stop_repeat();
        });
        let next = next.unwrap();
        EventDefault::player().send_message(
            comp!("Number: ") + next
            + comp!(" | Iter: ") + iterator
        );
    });
}
