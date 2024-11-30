use crate::api::items::loc::Location;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::{BlockType, TemplateBlock};
use crate::codetemplate::args::Item as DFItem;

impl Player {
    pub fn teleport(&self, location: Location) {
        push_block(TemplateBlock::player_action(
            "Teleport".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, location.as_item())
                .with_slot(25, crate::codetemplate::args::Item::block_tag("False", "Keep Velocity",
                                                                          "Teleport", BlockType::PlayerAction))
                .with_slot(26, DFItem::block_tag("False", "Keep Current Rotation",
                                                 "Teleport", BlockType::PlayerAction)),
        ))
    }

    pub fn teleport_v(&self, location: Location, keep_velocity: bool, keep_current_rotation: bool) {
        push_block(TemplateBlock::player_action(
            "SendMessage".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, location.as_item())
                .with_slot(25, DFItem::block_tag(if keep_velocity { "True" } else { "False" },
                                                 "Keep Velocity", "Teleport", BlockType::PlayerAction))
                .with_slot(26, DFItem::block_tag(if keep_current_rotation { "True" } else { "False" },
                                                 "Keep Current Rotation", "Teleport", BlockType::PlayerAction)),
        ))
    }
}