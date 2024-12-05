pub mod movement;

use crate::api::selections::Selection;
use crate::selections::ActionTarget;

#[derive(Debug, Copy, Clone)]
pub struct Entity;

impl ActionTarget for Entity {}

impl Selection for Entity {
    type Base = Entity;

    fn selection_mechanism(&self) {}
}
