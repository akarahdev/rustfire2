use crate::api::CURRENT_TEMPLATE;
use crate::api::items::component::Component;
use crate::api::items::VarItem;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, TemplateBlock};

pub struct Player;

impl Player {
    pub fn send_message<T: Into<Component>>(message: T) {
        CURRENT_TEMPLATE.lock().unwrap().blocks.push(
            TemplateBlock::player_action(
                "SendMessage".to_string(),
                "Selection".to_string(),
                ChestArgs::new()
                    .with_slot(0, message.into().as_item())
                    .with_slot(24, Item::block_tag("False", "Inherit Styles",
                                                   "SendMessage", BlockType::PlayerAction))
                    .with_slot(25, Item::block_tag("Add Spaces", "Text Value Merging",
                                                   "SendMessage", BlockType::PlayerAction))
                    .with_slot(26, Item::block_tag("Regular", "Alignment Mode",
                                                   "SendMessage", BlockType::PlayerAction))
            )
        )
    }
}