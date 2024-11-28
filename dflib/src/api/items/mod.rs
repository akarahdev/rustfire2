pub mod component;
pub mod number;
pub mod refs;
pub mod string;
pub mod intos;

use crate::codetemplate::args::Item;

pub trait VarItem: Clone {
    fn as_item(&self) -> Item;
}

#[macro_export]
macro_rules! num {
    ($t:expr) => { $crate::api::items::number::Number::new($t) }
}

#[macro_export]
macro_rules! str {
    ($t:expr) => { $crate::api::items::string::String::new($t) }
}

#[macro_export]
macro_rules! comp {
    ($t:expr) => { $crate::api::items::component::Component::new($t) }
}