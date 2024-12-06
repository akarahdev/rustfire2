pub mod filters;
pub mod groups;
pub mod targets;
pub mod stored;

pub use filters::*;
pub use groups::*;
pub use targets::*;
pub use stored::*;

use crate::items::{Cell, List, Location, Number, String, VarItem};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use crate::api::push_block;
use crate::core::args::{ChestArgs, GameValueData, TemplateItem};
use crate::core::template::TemplateBlock;

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

    fn cache(&self) -> Stored<Self::Base> 
    where <Self as Selection>::Base: Selection,
    Self: Deref {
        let _ = self.clone().deref();
        let list = List::<String>::new();
        list.append_list(List::from_item(
            TemplateItem::GameValue {
                data: GameValueData {
                    value: "Selection Target UUIDs",
                    target: "Default",
                }
            }
        ));
        push_block(TemplateBlock::select_object(
            "Reset".to_string(),
            ChestArgs::new()
        ));
        Stored(list, PhantomData)

    }
}
