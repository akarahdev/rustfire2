use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::{allocate_variable, push_block};
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, BracketDirection, BracketType, TemplateBlock};


pub struct ElseHandle;

impl ElseHandle {
    pub fn or_else<F: FnOnce()>(&self, or_else: F) {
        push_block(TemplateBlock::else_block());
        push_block(TemplateBlock::bracket(BracketDirection::Start, BracketType::Normal));
        or_else();
        push_block(TemplateBlock::bracket(BracketDirection::End, BracketType::Normal));
    }
}

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

pub struct Duration(u64);

impl Duration {
    pub fn ticks(ticks: u64) -> Duration {
        Duration(ticks)
    }

    pub fn seconds(seconds: u64) -> Duration {
        Duration(seconds * 20)
    }

    pub fn minutes(minutes: u64) -> Duration {
        Duration(minutes * 20 * 60)
    }
}

pub struct Control;

impl Control {
    pub fn wait(dur: Duration) {
        push_block(
            TemplateBlock::control(
                "Wait",
                ChestArgs::new()
                    .with_slot(0, Item::Number { data: NamedData { name: dur.0.to_string() } })
                    .with_slot(26, Item::block_tag("Ticks", "Time Unit",
                                                   "Wait", BlockType::Control)),
            )
        );
    }
}