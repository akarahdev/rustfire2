use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::codetemplate::template::{Template, TemplateBlock};

pub enum PlayerEvent {
    Join,
    Leave,
    RightClick,
    LeftClick,
    RightClickEntity,
    LeftClickEntity,
    Respawn
}

impl PlayerEvent {
    pub fn name(&self) -> String {
        match self {
            PlayerEvent::Join => "Join",
            PlayerEvent::Leave => "Leave",
            PlayerEvent::RightClick => "RightClick",
            PlayerEvent::LeftClick => "LeftClick",
            PlayerEvent::RightClickEntity => "RightClickEntity",
            PlayerEvent::LeftClickEntity => "LeftClickEntity",
            PlayerEvent::Respawn => "Respawn",
        }.to_string()
    }

    pub fn declare(&self, code: fn()) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::player_event(self.name()));
        code();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
}