use valence_nbt::Compound;
use valence_nbt::{Value};
use crate::api::{allocate_variable, push_block, Item as DFItem};
use crate::api::items::{TypedVarItem, VarItem};
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::TemplateBlock;

#[derive(Clone, Debug, Copy)]
pub struct Item(pub(crate) DFItem);
impl TypedVarItem for Item {}
impl VarItem for Item {
    fn as_item(&self) -> DFItem {
        self.0.clone()
    }

    fn from_item(item: DFItem) -> Self {
        Item(item)
    }

    fn default() -> Self {
        Item::new("minecraft:dirt")
    }
}

impl Item {
    pub fn new(id: &'static str) -> Item {
        let result = allocate_variable();
        push_block(
            TemplateBlock::set_variable(
                "SetItemType".to_string(),
                ChestArgs::new()
                    .with_slot(0, result.clone())
                    .with_slot(1, String::new(id).as_item())
            )
        );
        Item(result)
    }

    pub fn with_type(&self, id: String) -> Item {
        let result = allocate_variable();
        push_block(
            TemplateBlock::set_variable(
                "SetItemType".to_string(),
                ChestArgs::new()
                    .with_slot(0, result.clone())
                    .with_slot(1, self.as_item().clone())
                    .with_slot(2, id.as_item().clone())
            )
        );
        Item(result)
    }

    pub fn with_count(&self, count: Number) -> Item {
        let result = allocate_variable();
        push_block(
            TemplateBlock::set_variable(
                "SetItemAmount".to_string(),
                ChestArgs::new()
                    .with_slot(0, result.clone())
                    .with_slot(1, self.as_item().clone())
                    .with_slot(2, count.as_item().clone())
            )
        );
        Item(result)
    }
}