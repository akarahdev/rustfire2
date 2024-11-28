pub mod component;
pub mod loc;
pub mod number;
pub mod string;
pub mod intos;
pub mod list;
pub mod dict;
pub mod vec;
pub mod any;
pub mod item;
pub mod boolean;

use crate::codetemplate::args::Item;

pub trait VarItem: Clone {
    fn as_item(&self) -> Item;
    fn from_item(item: Item) -> Self;
}

pub trait TypedVarItem: VarItem {}

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

#[macro_export]
macro_rules! list {
    ($($t:expr),*) => {
        $crate::api::items::list::List::new_with_all(vec![$($t),*])
    }
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $value:expr),*) => {{
        let _lk = $crate::api::items::list::List::new_with_all(vec![$($key),*]);
        let _lv = $crate::api::items::list::List::new_with_all(vec![$($value),*]);
        $crate::api::items::dict::Dictionary::from_lists(_lk, _lv)
    }}
}