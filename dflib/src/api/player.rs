use crate::api::{push_block, CURRENT_TEMPLATE};
use crate::api::items::component::Component;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::selection::Selection;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, TemplateBlock};

#[derive(Debug, Copy, Clone)]
pub struct Player;

impl Selection for Player {
    type Base = Player;

    fn selection_mechanism(&self) -> Self::Base {
        Player
    }
}

impl Player {
    pub fn send_message(&self, message: Component) {
        push_block(TemplateBlock::player_action(
            "SendMessage".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, message.as_item())
                .with_slot(24, Item::block_tag("False", "Inherit Styles",
                                               "SendMessage", BlockType::PlayerAction))
                .with_slot(25, Item::block_tag("Add Spaces", "Text Value Merging",
                                               "SendMessage", BlockType::PlayerAction))
                .with_slot(26, Item::block_tag("Regular", "Alignment Mode",
                                               "SendMessage", BlockType::PlayerAction)),
        ))
    }

    pub fn send_action_bar(&self, message: Component) {
        push_block(TemplateBlock::player_action(
            "ActionBar".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, message.as_item())
                .with_slot(25, Item::block_tag("False", "Inherit Styles",
                                               "ActionBar", BlockType::PlayerAction))
                .with_slot(26, Item::block_tag("Add Spaces", "Text Value Merging",
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

    pub fn teleport(&self, location: Location) {
        push_block(TemplateBlock::player_action(
            "Teleport".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, location.as_item())
                .with_slot(25, Item::block_tag("False", "Keep Velocity",
                                               "Teleport", BlockType::PlayerAction))
                .with_slot(26, Item::block_tag("False", "Keep Current Rotation",
                                               "Teleport", BlockType::PlayerAction)),
        ))
    }

    pub fn teleport_v(&self, location: Location, keep_velocity: bool, keep_current_rotation: bool) {
        push_block(TemplateBlock::player_action(
            "SendMessage".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, location.as_item())
                .with_slot(25, Item::block_tag(if keep_velocity { "True" } else { "False" },
                                               "Keep Velocity", "Teleport", BlockType::PlayerAction))
                .with_slot(26, Item::block_tag(if keep_current_rotation { "True" } else { "False" },
                                               "Keep Current Rotation", "Teleport", BlockType::PlayerAction)),
        ))
    }
}