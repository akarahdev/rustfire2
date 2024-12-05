use crate::api::items::item::Item;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::core::args::ChestArgs;
use crate::core::template::{BracketDirection, BracketType, TemplateBlock};

impl Player {
    pub fn is_holding<F: FnOnce()>(&self, item: Item, if_true: F) {
        push_block(TemplateBlock::if_player(
            "IsHolding".to_string(),
            "Selection".to_string(),
            ChestArgs::new().with_slot(0, item.as_item()),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Normal,
        ));
        if_true();
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Normal,
        ));
    }
}
