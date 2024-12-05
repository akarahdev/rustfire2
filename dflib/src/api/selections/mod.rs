pub mod targets;
pub mod filters;
pub mod groups;

pub use targets::*;
pub use filters::*;
pub use groups::*;

use crate::items::{Location, Number};
use std::fmt::Debug;

pub trait ActionTarget: Selection {}

pub trait Selection: Clone + Debug {
    type Base;

    fn selection_mechanism(&self);

    fn random(&self) -> FilterRandomly<Self> {
        FilterRandomly(self.clone(), Number::new("1"))
    }

    fn random_amount(&self, amount: Number) -> FilterRandomly<Self> {
        FilterRandomly(self.clone(), amount)
    }

    fn distance(&self, location: Location) -> FilterDistance<Self> {
        FilterDistance(self.clone(), location, Number::new("1"))
    }

    fn distance_amount(&self, location: Location, amount: Number) -> FilterDistance<Self> {
        FilterDistance(self.clone(), location, amount)
    }
}
