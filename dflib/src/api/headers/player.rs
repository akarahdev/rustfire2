use crate::api::{push_block, start_new_template};
use crate::codetemplate::template::TemplateBlock;

#[derive(Copy, Clone)]
pub enum PlayerEvent {
    Join,
    Leave,
    RightClick,
    LeftClick,
    RightClickEntity,
    LeftClickEntity,
    Respawn,
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
        }
        .to_string()
    }

    pub fn declare<F: FnOnce() + Send + 'static>(self, code: F) {
        start_new_template(move || {
            push_block(TemplateBlock::player_event(self.name().to_string()));
            code();
        });
    }
}
