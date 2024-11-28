use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::api::player::Player;
use crate::api::selection::EventDefault;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, Template, TemplateBlock};

pub struct PlayerEvent;

impl PlayerEvent {
    pub fn join<F: FnOnce(EventDefault<Player>)>(f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::player_event("Join".to_string()));
        f(EventDefault(Player));
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
}