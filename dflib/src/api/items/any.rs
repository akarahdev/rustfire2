use crate::api::items::{TypedVarItem, VarItem};
use crate::codetemplate::args::{Item, NamedData};

#[derive(Debug, Clone)]
pub struct Any(pub(crate) Item);

impl VarItem for Any {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Any(item)
    }
}

impl<T: TypedVarItem> From<T> for Any {
    fn from(value: T) -> Self {
        Any(value.as_item())
    }
}

impl Any {
    pub fn empty() -> Self {
        Any(Item::Number { data: NamedData { name: "0".to_string() }})
    }

    pub fn from_value<V: VarItem>(v: V) -> Any {
        Any(v.as_item())
    }
    
    pub fn cast<T: VarItem>(&self) -> T {
        T::from_item(self.0.clone())
    }
}