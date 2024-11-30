pub mod targets;

use std::fmt::Debug;
use crate::api::items::number::Number;
use crate::api::selections::targets::FilterRandomly;

pub trait Selection: Clone + Debug {
    type Base;

    fn selection_mechanism(&self);

    fn filter_random(&self) -> FilterRandomly<Self> {
        FilterRandomly(self.clone(), Number::new("1"))
    }

    fn filter_random_amount(&self, amount: Number) -> FilterRandomly<Self> {
        FilterRandomly(self.clone(), amount)
    }
}