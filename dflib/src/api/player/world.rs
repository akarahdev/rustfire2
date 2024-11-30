use crate::api::items::item::Item;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::TemplateBlock;

impl Player {
    pub fn launch_proj(&self, projectile: Item) {
        push_block(TemplateBlock::player_action(
            "LaunchProj".to_string(),
            "Selection".to_string(),
            ChestArgs::new()
                .with_slot(0, projectile.as_item())
        ))
    }
}