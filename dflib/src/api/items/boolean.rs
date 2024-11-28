use std::ops::Add;
use crate::api::{allocate_variable, push_block};
use crate::api::flow::ElseHandle;
use crate::api::items::{TypedVarItem, VarItem};
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, BracketDirection, BracketType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct Boolean(pub(crate) Item);
impl TypedVarItem for Boolean {}

impl VarItem for Boolean {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Boolean(item)
    }
}

impl Boolean {
    pub fn new(raw: bool) -> Boolean {
        Boolean(Item::String { data: NamedData { name: raw.to_string() }})
    }

    pub fn if_true<F: FnOnce()>(&self, f: F) -> ElseHandle {
        push_block(TemplateBlock::if_variable(
            "=".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, Item::String { data: NamedData { name: "[rf/bool]true".to_string() }})
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        f();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
        ElseHandle
    }
}