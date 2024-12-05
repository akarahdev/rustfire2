use crate::api::{push_block, start_new_template};
use crate::core::template::TemplateBlock;

#[derive(Copy, Clone)]
pub enum PlayerEvent {
    Join,
    Leave,
    Command,
    PackLoad,
    PackDecline,
    ViewVIPPerks,

    RightClick,
    LeftClick,
    RightClickEntity,
    LeftClickEntity,
    LoadCrossbow,

    PlaceBlock,
    BreakBlock,
    SwapHands,
    ChangeSlot,

    Respawn,
    TameEntity,
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
            PlayerEvent::LoadCrossbow => "LoadCrossbow",
            PlayerEvent::PlaceBlock => "PlaceBlock",
            PlayerEvent::BreakBlock => "BreakBlock",
            PlayerEvent::SwapHands => "SwapHands",
            PlayerEvent::ChangeSlot => "ChangeSlot",
            PlayerEvent::TameEntity => "TameEntity",
            PlayerEvent::Command => "Command",
            PlayerEvent::PackLoad => "PackLoad",
            PlayerEvent::PackDecline => "PackDecline",
            PlayerEvent::ViewVIPPerks => "ViewVIPPerks",
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
