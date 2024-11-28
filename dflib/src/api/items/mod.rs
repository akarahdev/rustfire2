pub mod component;
pub mod number;
pub mod refs;
pub mod string;
pub mod intos;
pub mod list;
pub mod dict;

use crate::codetemplate::args::Item;

pub trait VarItem: Clone {
    fn as_item(&self) -> Item;
    fn from_item(item: Item) -> Self;
}

#[macro_export]
macro_rules! num {
    ($($t:tt)*) => { $crate::api::items::number::Number::new(stringify!($($t)*)) }
}

#[macro_export]
macro_rules! str {
    ($t:expr) => { $crate::api::items::string::String::new($t) }
}

#[macro_export]
macro_rules! comp {
    ($t:expr) => { $crate::api::items::component::Component::new($t) }
}