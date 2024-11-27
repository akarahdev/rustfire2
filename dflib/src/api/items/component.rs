use std::sync::atomic::Ordering;
use crate::api::{allocate_variable, push_block, CURRENT_TEMPLATE, VAR_INDEX};
use crate::api::items::refs::Ref;
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item, NamedData, VarData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Clone)]
pub struct Component(pub(crate) Item);

impl VarItem for Component {
    fn as_item(&self) -> Item {
        self.0.clone()
    }
}

impl From<Ref<Component>> for Component {
    fn from(value: Ref<Component>) -> Self {
        (*value).clone()
    }
}

impl Component {
    pub fn new(raw: &str) -> Component {
        Component(Item::Component { data: NamedData { name: raw.to_string() }})
    }

    pub fn cast<T: VarItem>(value: T) -> Ref<Component> {
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
        Ref(Component(result))
    }
}
