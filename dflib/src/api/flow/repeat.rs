use crate::api::items::dict::Dictionary;
use crate::api::items::list::List;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::{allocate_variable, push_block};
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, BracketDirection, BracketType, TemplateBlock};

pub struct Repeat;

impl Repeat {
    pub fn forever<F: FnOnce()>(code: F) {
        push_block(TemplateBlock::repeat("Forever", ChestArgs::new()));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code();
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }

    pub fn multiple<F: FnOnce(Number)>(times: Number, code: F) {
        let idx = allocate_variable();
        push_block(TemplateBlock::repeat(
            "Multiple",
            ChestArgs::new()
                .with_slot(0, idx.clone())
                .with_slot(1, times.as_item()),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code(Number(idx.clone()));
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }

    pub fn range<F: FnOnce(Number)>(start: Number, end: Number, step: Number, code: F) {
        let idx = allocate_variable();
        push_block(TemplateBlock::repeat(
            "Range",
            ChestArgs::new()
                .with_slot(0, idx.clone())
                .with_slot(1, start.as_item())
                .with_slot(2, end.as_item())
                .with_slot(3, step.as_item()),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code(Number(idx.clone()));
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }

    pub fn for_each<T: VarItem, F: FnOnce(T)>(list: List<T>, code: F) {
        let idx = allocate_variable();
        push_block(TemplateBlock::repeat(
            "ForEach",
            ChestArgs::new()
                .with_slot(0, idx.clone())
                .with_slot(1, list.as_item())
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "True",
                        "Allow List Changes",
                        "ForEach",
                        BlockType::Repeat,
                    ),
                ),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code(T::from_item(idx.clone()));
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }

    pub fn for_each_entry<K: VarItem, V: VarItem, F: FnOnce(K, V)>(
        dict: Dictionary<K, V>,
        code: F,
    ) {
        let k = allocate_variable();
        let v = allocate_variable();
        push_block(TemplateBlock::repeat(
            "ForEachEntry",
            ChestArgs::new()
                .with_slot(0, k.clone())
                .with_slot(1, v.clone())
                .with_slot(2, dict.as_item())
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "True",
                        "Allow List Changes",
                        "ForEach",
                        BlockType::Repeat,
                    ),
                ),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code(K::from_item(k), V::from_item(v));
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }

    pub fn grid<F: FnOnce(Location)>(start: Location, end: Location, code: F) {
        let idx = allocate_variable();
        push_block(TemplateBlock::repeat(
            "Grid",
            ChestArgs::new()
                .with_slot(0, idx.clone())
                .with_slot(1, start.as_item())
                .with_slot(2, end.as_item()),
        ));
        push_block(TemplateBlock::bracket(
            BracketDirection::Start,
            BracketType::Sticky,
        ));
        code(Location(idx.clone()));
        push_block(TemplateBlock::bracket(
            BracketDirection::End,
            BracketType::Sticky,
        ));
    }
}
