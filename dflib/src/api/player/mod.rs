pub mod data;
pub mod items;
pub mod comms;
pub mod movement;
pub mod world;
pub mod cond;

use crate::api::selections::Selection;

#[derive(Debug, Copy, Clone)]
pub struct Player;

impl Selection for Player {
    type Base = Player;

    fn selection_mechanism(&self) {

    }
}