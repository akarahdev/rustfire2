use std::ops::Add;
use crate::api::{allocate_variable, push_block};
use crate::api::items::{TypedVarItem, VarItem};
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct Component(pub(crate) Item);
impl TypedVarItem for Component {}

impl VarItem for Component {
    fn as_item(&self) -> Item {
        self.0.clone()
    }

    fn from_item(item: Item) -> Self {
        Component(item)
    }
}

impl Component {
    pub fn new(raw: &str) -> Component {
        Component(Item::Component { data: NamedData { name: raw.to_string() }})
    }

    pub fn cast<T: VarItem>(value: T) -> Component {
        let result = allocate_variable();
        push_block(TemplateBlock::set_variable(
            "StyledText".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, value.clone().as_item())
                .with_slot(25, Item::block_tag("True", "Inherit Styles",
                                               "StyledText", BlockType::SetVariable))
                .with_slot(26, Item::block_tag("No Spaces", "Text Value Merging",
                                               "StyledText", BlockType::SetVariable))
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
                .with_slot(25, Item::block_tag("True", "Inherit Styles",
                                               "StyledText", BlockType::SetVariable))
                .with_slot(26, Item::block_tag("No Spaces", "Text Value Merging",
                                               "StyledText", BlockType::SetVariable))
        ));
        Component(result)
    }
}