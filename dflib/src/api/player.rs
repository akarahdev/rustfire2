use std::marker::PhantomData;
use crate::api::{allocate_variable, push_block, CURRENT_TEMPLATE};
use crate::api::flow::ElseHandle;
use crate::api::items::any::Any;
use crate::api::items::boolean::Boolean;
use crate::api::items::component::Component;
use crate::api::items::dict::Dictionary;
use crate::api::items::item::Item;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::api::items::VarItem;
use crate::api::selection::Selection;
use crate::codetemplate::args::{ChestArgs, Item as DFItem, NamedData, VarData};
use crate::codetemplate::template::{BlockType, BracketDirection, BracketType, TemplateBlock};

#[derive(Debug, Copy, Clone)]
pub struct Player;

impl Selection for Player {
    type Base = Player;

    fn selection_mechanism(&self) {

    }
}

impl Player {
    pub fn game_data(&self) -> Dictionary<String, Any> {
        Dictionary(
            DFItem::Variable { data: VarData { name: "rf/%uuid/d".to_string(), scope: "unsaved".to_string() } },
            PhantomData
        )
    }

    pub fn saved_data(&self) -> Dictionary<String, Any> {
        Dictionary(
            DFItem::Variable { data: VarData { name: "rf/%uuid/sd".to_string(), scope: "saved".to_string() } },
            PhantomData
        )
    }
}

impl Player {
    pub fn give_item(&self, item: Item) {
        push_block(TemplateBlock::player_action(
            "GiveItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
        ))
    }
    
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

    pub fn teleport(&self, location: Location) {
        push_block(TemplateBlock::player_action(
            "Teleport".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, location.as_item())
                .with_slot(25, DFItem::block_tag("False", "Keep Velocity",
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
    
    pub fn launch_proj(&self, projectile: Item) {
        push_block(TemplateBlock::player_action(
            "LaunchProj".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, projectile.as_item())
        ))
    }
    
    pub fn is_holding(&self, item: Item) -> Boolean {
        let mut result = allocate_variable();
        push_block(TemplateBlock::if_player(
            "IsHolding".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        push_block(TemplateBlock::set_variable(
            "=".to_string(),
            ChestArgs::new()
                .with_slot(0, result.clone())
                .with_slot(1, DFItem::String { data: NamedData { name: "[rf/bool]true".to_string() }})
        ));
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
        Boolean(result)
    }
}