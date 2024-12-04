use std::ops::Add;
use crate::api::{allocate_variable, push_block};
use crate::api::items::{set_variable, TypedVarItem, VarItem};
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone, Copy)]
pub struct String(pub(crate) Item);
impl TypedVarItem for String {}

impl VarItem for String {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        String(item)
    }

    fn default() -> Self {
        String::new("")
    }
}

impl String {
    pub fn new(raw: &'static str) -> String {
        String(Item::String { data: NamedData { name: raw }})
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

set_variable! {
    impl String; fn (replace => "ReplaceString") -> String;
    arg from: String;
    arg to: String;

    tag "Replacement Type" => "All occurences";
    tag "Regular Expression" => "Disable";
}

set_variable! {
    impl String; fn (replace_regex => "ReplaceString") -> String;
    arg from: String;
    arg to: String;

    tag "Replacement Type" => "All occurences";
    tag "Regular Expression" => "Enable";
}