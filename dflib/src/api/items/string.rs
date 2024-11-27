use crate::api::items::VarItem;
use crate::codetemplate::args::{Item, NamedData};

#[derive(Debug, Clone)]
pub struct String(pub(crate) Item);

impl VarItem for String {
    fn as_item(&self) -> Item {
        self.0.clone()
    }
}

impl String {
    pub fn new(raw: &str) -> String {
        String(Item::Component { data: NamedData { name: raw.to_string() }})
    }
}
