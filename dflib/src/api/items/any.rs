use crate::api::items::number::Number;
use crate::api::items::{TypedVarItem, VarItem};
use crate::core::args::{NamedData, TemplateItem};

#[derive(Debug, Clone, Copy)]
pub struct Any(pub(crate) TemplateItem);

impl VarItem for Any {
    fn as_item(&self) -> TemplateItem {
        self.0.clone()
    }

    fn from_item(item: TemplateItem) -> Self {
        Any(item)
    }

    fn default() -> Self {
        Any::from_item(Number::default().as_item())
    }
}

impl Any {
    pub fn empty() -> Self {
        Any(TemplateItem::Number {
            data: NamedData { name: "0" },
        })
    }

    pub fn from_item(item: TemplateItem) -> Self {
        Any(item)
    }

    pub fn from_value<V: VarItem>(v: V) -> Any {
        Any(v.as_item())
    }

    pub fn cast<T: VarItem>(&self) -> T {
        T::from_item(self.0.clone())
    }
}
