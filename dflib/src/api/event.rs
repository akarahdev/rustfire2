use crate::api::{push_block, CURRENT_TEMPLATE, TEMPLATE_REPOSITORY};
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, Template, TemplateBlock};

pub struct PlayerEvent;

impl PlayerEvent {
    pub fn join<F: FnOnce()>(f: F) {
        CURRENT_TEMPLATE.lock().unwrap().blocks = vec![];
        push_block(TemplateBlock::player_event("Join".to_string()));
        push_block(
            TemplateBlock::select_object(
                "EventTarget".to_string(),
                ChestArgs::new()
                    .with_slot(26, Item::block_tag("Default", "Event Target",
                                                   "EventTarget", BlockType::SelectObject)),
            )
        );
        f();
        TEMPLATE_REPOSITORY.lock().unwrap().push(Template { blocks: CURRENT_TEMPLATE.lock().unwrap().blocks.clone() });
    }
}