use std::marker::PhantomData;
use crate::api::items::{TypedVarItem, VarItem};
use crate::api::{allocate_variable, push_block};
use crate::api::items::number::Number;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone, Copy)]
pub struct List<T: VarItem>(Item, PhantomData<T>);
impl<T: VarItem> TypedVarItem for List<T> {}

impl<T: VarItem> VarItem for List<T> {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        List(item, PhantomData)
    }

    fn default() -> Self {
        Self::new()
    }
}

impl<T: VarItem> List<T> {
    pub fn new() -> List<T> {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "CreateList".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone()),
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
            args,
        ));
        List(result, PhantomData)
    }

    pub fn append(&self, value: T) -> &Self {
        push_block(TemplateBlock::set_variable(
            "AppendValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, value.as_item()),
        ));
        self
    }

    pub fn append_list(&self, value: List<T>) -> &Self {
        push_block(TemplateBlock::set_variable(
            "AppendList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, value.as_item()),
        ));
        self
    }

    pub fn pop(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "PopListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item()),
        ));
        self
    }

    pub fn set(&self, index: Number, value: T) -> &Self {
        push_block(TemplateBlock::set_variable(
            "SetListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, index.as_item())
                .with_slot(2, value.as_item()),
        ));
        self
    }

    pub fn get(&self, index: Number) -> T {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "GetListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item())
                .with_slot(2, index.as_item()),
        ));
        T::from_item(result)
    }

    pub fn get_value_index(&self, value: T) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "PopListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item())
                .with_slot(2, value.as_item()),
        ));
        Number(result)
    }

    pub fn len(&self) -> Number {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "ListLength".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item()),
        ));
        Number(result)
    }

    pub fn insert(&self, index: Number, value: T) -> &Self {
        push_block(TemplateBlock::set_variable(
            "InsertListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, index.as_item())
                .with_slot(2, value.as_item()),
        ));
        self
    }

    pub fn remove_value(&self, value: T) -> &Self {
        push_block(TemplateBlock::set_variable(
            "RemoveListValue".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, value.as_item()),
        ));
        self
    }

    pub fn remove(&self, index: Number) -> &Self {
        push_block(TemplateBlock::set_variable(
            "RemoveListIndex".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, index.as_item()),
        ));
        self
    }

    pub fn dedup(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "DedupList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item()),
        ));
        self
    }

    pub fn trim(&self, start: Number, end: Number) -> &Self {
        push_block(TemplateBlock::set_variable(
            "TrimList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(1, start.as_item())
                .with_slot(2, end.as_item()),
        ));
        self
    }

    pub fn sort(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "SortList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
                .with_slot(26, Item::block_tag("Ascending", "Sort Order",
                                               "SortList", BlockType::SetVariable)),
        ));
        self
    }

    pub fn reverse(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "ReverseList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
        ));
        self
    }

    pub fn randomize(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "RandomizeList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
        ));
        self
    }

    pub fn flatten(&self) -> &Self {
        push_block(TemplateBlock::set_variable(
            "FlattenList".to_string(),
            ChestArgs::new()
                .with_slot(0, self.as_item())
        ));
        self
    }
}