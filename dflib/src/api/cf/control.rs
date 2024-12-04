use crate::api::cf::time::Duration;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::push_block;
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

pub struct Control;

impl Control {
    pub fn wait(dur: Number) {
        push_block(TemplateBlock::control(
            "Wait",
            ChestArgs::new()
                .with_slot(0, dur.as_item())
                .with_slot(26, Item::block_tag("Ticks", "Time Unit",
                                               "Wait", BlockType::Control)),
        ));
    }

    pub fn return_from_function() {
        push_block(TemplateBlock::control(
            "Return",
            ChestArgs::new(),
        ));
    }

    pub fn end_thread() {
        push_block(TemplateBlock::control(
            "End",
            ChestArgs::new(),
        ));
    }

    pub fn skip_iteration() {
        push_block(TemplateBlock::control(
            "Skip",
            ChestArgs::new(),
        ));
    }

    pub fn stop_repeat() {
        push_block(TemplateBlock::control(
            "StopRepeat",
            ChestArgs::new(),
        ));
    }
}