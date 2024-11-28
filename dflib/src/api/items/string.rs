use std::ops::Add;
use crate::api::{allocate_variable, push_block};
use crate::api::items::component::Component;
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct String(pub(crate) Item);

impl VarItem for String {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        String(item)
    }
}

impl String {
    pub fn new(raw: &str) -> String {
        String(Item::Component { data: NamedData { name: raw.to_string() }})
    }
}

impl<T: VarItem> Add<T> for String {
    type Output = String;

    fn add(self, rhs: T) -> Self::Output {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "String".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item())
                .with_slot(2, rhs.as_item())
                .with_slot(26, Item::block_tag("No Spaces", "Text Value Merging",
                                               "StyledText", BlockType::SetVariable))
        ));
        String(result)
    }
}