use crate::api::entity::Entity;
use crate::api::items::loc::Location;
use crate::api::items::VarItem;
use crate::api::push_block;
use crate::core::args::ChestArgs;
use crate::core::args::TemplateItem as DFItem;
use crate::core::template::{BlockType, TemplateBlock};

impl Entity {
    pub fn teleport(&self, loc: Location) {
        push_block(TemplateBlock::entity_action(
            "Teleport".to_string(),
            "Selection".to_string(),
            ChestArgs::new().with_slot(0, loc.as_item()).with_slot(
                26,
                DFItem::block_tag(
                    "False",
                    "Keep Current Rotation",
                    "Teleport",
                    BlockType::EntityAction,
                ),
            ),
        ))
    }
}
