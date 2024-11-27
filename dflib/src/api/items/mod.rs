pub mod component;
pub mod number;
pub mod refs;
pub mod string;
pub mod intos;

use crate::codetemplate::args::Item;

pub trait VarItem: Clone {
    fn as_item(&self) -> Item;
}