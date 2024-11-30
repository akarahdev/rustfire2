use crate::api::items::number::Number;
use crate::api::{allocate_variable, push_block};
use crate::api::items::VarItem;
use crate::codetemplate::args::ChestArgs;
use crate::codetemplate::template::{BracketDirection, BracketType, TemplateBlock};

pub struct Repeat;

impl Repeat {
    pub fn forever<F: FnOnce()>(code: F) {
        push_block(TemplateBlock::repeat("Forever", ChestArgs::new()));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Sticky));
        code();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Sticky));
    }

    pub fn multiple<F: FnOnce(Number)>(times: Number, code: F) {
        let idx = allocate_variable();
        push_block(TemplateBlock::repeat(
            "Multiple",
            ChestArgs::new()
                .with_slot(0, idx.clone())
                .with_slot(1, times.as_item()),
        ));
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Sticky));
        code(Number(idx.clone()));
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Sticky));
    }
}