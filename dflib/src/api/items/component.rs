use crate::api::items::{TypedVarItem, VarItem};
use crate::api::{allocate_variable, push_block};
use crate::core::args::{ChestArgs, NamedData, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Component(pub(crate) TemplateItem);
impl TypedVarItem for Component {}

impl VarItem for Component {
    fn as_item(&self) -> TemplateItem {
        self.0.clone()
    }

    fn from_item(item: TemplateItem) -> Self {
        Component(item)
    }

    fn default() -> Self {
        Component::new("")
    }
}

impl Component {
    pub fn new(raw: &'static str) -> Component {
        Component(TemplateItem::Component {
            data: NamedData { name: raw },
        })
    }

    pub fn cast<T: VarItem>(value: T) -> Component {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "StyledText".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, value.clone().as_item())
                .with_slot(
                    25,
                    TemplateItem::block_tag(
                        "True",
                        "Inherit Styles",
                        "StyledText",
                        BlockType::SetVariable,
                    ),
                )
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
        Component(result)
    }
}

impl<T: VarItem> Add<T> for Component {
    type Output = Component;

    fn add(self, rhs: T) -> Self::Output {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "StyledText".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, self.as_item())
                .with_slot(2, rhs.as_item())
                .with_slot(
                    25,
                    TemplateItem::block_tag(
                        "True",
                        "Inherit Styles",
                        "StyledText",
                        BlockType::SetVariable,
                    ),
                )
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
        Component(result)
    }
}
