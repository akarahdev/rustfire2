use crate::api::player::Player;
use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::api::entity::Entity;
use crate::api::selections::targets::EventDefault;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, Template, TemplateBlock};

pub struct Functions;

impl Functions {
    pub fn declare<F: FnOnce()>(name: &str, f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::function(name.to_string()));
        f();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }

    pub fn call(name: &str) {
        push_block(TemplateBlock::call_function(name.to_string()));
    }
}

pub struct Processes;

impl Processes {
    pub fn declare<F: FnOnce()>(name: &str, f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::process(name.to_string()));
        f();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
    
    pub fn call(name: &str) {
        push_block(TemplateBlock::start_process(name.to_string()));
    }
}

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

pub enum EntityEvent {
    Teleport
}

impl EntityEvent {
    pub fn name(&self) -> String {
        match self {
            EntityEvent::Teleport => "Teleport",
        }.to_string()
    }

    pub fn declare(&self, code: fn()) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::entity_event(self.name()));
        code();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
}