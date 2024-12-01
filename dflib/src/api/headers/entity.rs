use crate::api::{push_block, start_new_template, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::codetemplate::template::{Template, TemplateBlock};

pub enum EntityEvent {
    Teleport
}

impl EntityEvent {
    pub fn name(&self) -> String {
        match self {
            EntityEvent::Teleport => "Teleport",
        }.to_string()
    }

    pub fn declare(self, code: fn()) {
        start_new_template(move || {
            push_block(TemplateBlock::entity_event(self.name().to_string()));
            code();
        });
    }
}