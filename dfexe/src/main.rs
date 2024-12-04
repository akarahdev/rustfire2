use rustfire::api::headers::player::PlayerEvent;
use rustfire::api::selections::targets::EventDefault;
use rustfire::{comp, num, registry};
use rustfire::api::cf::control::Control;
use rustfire::api::cf::repeat::Repeat;
use rustfire::api::items::cell::Cell;
use rustfire::api::items::list::List;
use rustfire::api::items::loc::Location;
use rustfire::api::items::particle::Particle;
use rustfire::api::items::VarItem;
use rustfire::api::items::vec::Vector;
use rustfire::std::iter::Iterator;

registry!({
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    EventDefault::player().send_message(comp!("Hello world!"));

    let list = List::new();
    list.append(Location::new_const(10.0, 51.0, 10.0));
    list.append(Location::new_const(10.0, 52.0, 10.0));
    list.append(Location::new_const(10.0, 53.0, 10.0));
    list.append(Location::new_const(10.0, 54.0, 10.0));
    list.append(Location::new_const(11.0, 54.0, 10.0));
    list.append(Location::new_const(12.0, 54.0, 10.0));
    list.append(Location::new_const(13.0, 54.0, 10.0));
    list.append(Location::new_const(13.0, 54.0, 11.0));
    list.append(Location::new_const(13.0, 54.0, 12.0));
    list.append(Location::new_const(13.0, 54.0, 13.0));
    list.append(Location::new_const(13.0, 54.0, 14.0));

    let idx = Cell::wrap(num!(0));

    list.iter()
        .map(|loc| loc.with_y(loc.y() + num!(10)))
        .for_each(|loc| {
            idx.set(idx.into_inner() + num!(1));
            EventDefault::player().particle(
                Particle::new("Cloud")
                    .with_motion(Vector::new_const(0.0, 0.0, 0.0)),
                loc,
            );
            Control::wait(idx.into_inner());
        });
}
