use rustfire::flow::Repeat;
use rustfire::headers::PlayerEvent;
use rustfire::items::{num, Component, Item, Location, Number};
use rustfire::plot::Plot;
use rustfire::registry;
use rustfire::selections::targets::EventDefault;
use rustfire::selections::{AllOf, Selection};

registry!(main -> {
    PlayerEvent::Join.declare(on_join);
});

pub fn on_join() {
    Repeat::range(num!(50), num!(70), num!(5), |value| {
        Plot::set_region(
            Item::new("air"),
            Location::new_const(0.0, 50.0, 0.0)
                .with_y(value),
            Location::new_const(100.0, 50.0, 100.0)
                .with_y(value + num!(5)),
        );
    });
    let seed = Number::random_decimal(num!(0), num!(10000000));
    Repeat::grid(
        Location::new_const(0.0, 50.0, 0.0),
        Location::new_const(10.0, 50.0, 10.0),
        |loc| {
            let noise = Number::perlin(loc, num!(1), num!(1), num!(1.5), num!(0.5), seed);
            let shifted = loc.with_y(loc.y() + (noise * num!(12)));
            let dirt = shifted.with_y(shifted.y() - num!(4));
            let stone = shifted.with_y(shifted.y() - num!(60));
            Plot::set_region(Item::new("minecraft:stone"), stone, shifted);
            Plot::set_region(Item::new("minecraft:dirt"), dirt, shifted);
            Plot::set_block(Item::new("minecraft:grass_block"), shifted);
        },
    );
}
