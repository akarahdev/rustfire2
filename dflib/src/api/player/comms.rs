use crate::api::items::component::Component;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::{BlockType, TemplateBlock};
use crate::codetemplate::args::Item as DFItem;

impl Player {
    pub fn send_message(&self, message: Component) {
        push_block(TemplateBlock::player_action(
            "SendMessage".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, message.as_item())
                .with_slot(24, DFItem::block_tag("False", "Inherit Styles",
                                                                          "SendMessage", BlockType::PlayerAction))
                .with_slot(25, DFItem::block_tag("Add Spaces", "Text Value Merging",
                                                 "SendMessage", BlockType::PlayerAction))
                .with_slot(26, DFItem::block_tag("Regular", "Alignment Mode",
                                                 "SendMessage", BlockType::PlayerAction)),
        ))
    }

    pub fn send_action_bar(&self, message: Component) {
        push_block(TemplateBlock::player_action(
            "ActionBar".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, message.as_item())
                .with_slot(25, DFItem::block_tag("False", "Inherit Styles",
                                                 "ActionBar", BlockType::PlayerAction))
                .with_slot(26, DFItem::block_tag("Add Spaces", "Text Value Merging",
                                                 "ActionBar", BlockType::PlayerAction))
        ))
    }

    pub fn send_title(&self, title: Component, subtitle: Component,
                      duration: Number, fade_in: Number, fade_out: Number) {
        push_block(TemplateBlock::player_action(
            "SendTitle".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, title.as_item())
                .with_slot(0, subtitle.as_item())
                .with_slot(0, duration.as_item())
                .with_slot(0, fade_in.as_item())
                .with_slot(0, fade_out.as_item())
        ))
    }
}