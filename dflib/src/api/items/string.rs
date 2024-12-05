use crate::api::items::{set_variable, TypedVarItem, VarItem};
use crate::api::{allocate_variable, push_block};
use crate::core::args::{ChestArgs, TemplateItem, NamedData};
use crate::core::template::{BlockType, TemplateBlock};
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct String(pub(crate) TemplateItem);
impl TypedVarItem for String {}

impl VarItem for String {
    fn as_item(&self) -> TemplateItem {
        self.0.clone()
    }

    fn from_item(item: TemplateItem) -> Self {
        String(item)
    }

    fn default() -> Self {
        String::new("")
    }
}

impl String {
    pub fn new(raw: &'static str) -> String {
        String(TemplateItem::String {
            data: NamedData { name: raw },
        })
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
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "No Spaces",
                        "Text Value Merging",
                        "StyledText",
                        BlockType::SetVariable,
                    ),
                ),
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
