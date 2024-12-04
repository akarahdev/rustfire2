use crate::api::push_block;
use crate::codetemplate::template::{BracketDirection, BracketType, TemplateBlock};

pub struct ElseHandle;

impl ElseHandle {
    pub fn or_else<F: FnOnce()>(&self, or_else: F) {
        push_block(TemplateBlock::else_block());
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Normal,
        ));
        or_else();
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Normal,
        ));
    }
}
