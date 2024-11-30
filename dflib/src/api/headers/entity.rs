use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
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

    pub fn declare(&self, code: fn()) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::entity_event(self.name()));
        code();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
}