use std::marker::PhantomData;
use crate::api::items::string::String;
use crate::api::items::{TypedVarItem, VarItem};
use crate::api::{allocate_variable, push_block};
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct List<T: VarItem>(Item, PhantomData<T>);
impl<T: VarItem> TypedVarItem for List<T> {}

impl<T: VarItem> VarItem for List<T> {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        List(item, PhantomData)
    }
}

impl<T: VarItem> List<T> {
    pub fn new() -> List<T> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "CreateList".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
        ));
        List(result, PhantomData)
    }

    pub fn new_with_all(items: Vec<T>) -> List<T> {
        let result = allocate_variable();
        let mut args = ChestArgs::new();
        args = args.with_slot(0, result.clone());
        for item in items.iter().enumerate() {
            if item.0 > 26 {
                panic!("too big for one CreateList chest");
            }
            args = args.with_slot((item.0 + 1) as u8, item.1.as_item());
        }
        push_block(TemplateBlock::set_variable(
            "CreateList".to_string(),
            args
        ));
        List(result, PhantomData)
    }

    pub fn append(&self, value: T) -> &Self {
        push_block(TemplateBlock::set_variable(
            "AppendValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, value.as_item())
        ));
        self
    }
}