use crate::api::items::item::Item;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::TemplateBlock;

impl Player {
    pub fn give_item(&self, item: Item) {
        push_block(TemplateBlock::player_action(
            "GiveItems".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, item.as_item())
        ))
    }
}