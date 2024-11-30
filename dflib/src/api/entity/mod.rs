pub mod movement;

use crate::api::selections::Selection;

#[derive(Debug, Copy, Clone)]
pub struct Entity;

impl Selection for Entity {
    type Base = Entity;

    fn selection_mechanism(&self) {

    }
}