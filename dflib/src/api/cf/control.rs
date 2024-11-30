use crate::api::cf::time::Duration;
use crate::api::push_block;
use crate::codetemplate::args::{ChestArgs, Item, NamedData};
use crate::codetemplate::template::{BlockType, TemplateBlock};

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