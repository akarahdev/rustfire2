use crate::api::push_block;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use crate::impl_deref_for_sel;
use crate::items::{Location, Number, VarItem};
use crate::selections::Selection;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct FilterRandomly<C: Selection>(pub(crate) C, pub(crate) Number);

impl<C: Selection + Deref<Target = <C as Selection>::Base>> Deref for FilterRandomly<C> {
    type Target = <C as Selection>::Base;

    fn deref(&self) -> &Self::Target {
        let r = self.0.deref();
        self.selection_mechanism();
        r
    }
}

impl<C: Selection> Selection for FilterRandomly<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "FilterRandom".to_string(),
            ChestArgs::new().with_slot(0, self.1.as_item()),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct FilterDistance<C: Selection>(pub(crate) C, pub(crate) Location, pub(crate) Number);

impl<C: Selection + Deref<Target = <C as Selection>::Base>> Deref for FilterDistance<C> {
    type Target = <C as Selection>::Base;

    fn deref(&self) -> &Self::Target {
        let r = self.0.deref();
        self.selection_mechanism();
        r
    }
}

impl<C: Selection> Selection for FilterDistance<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "FilterDistance".to_string(),
            ChestArgs::new()
                .with_slot(0, self.1.as_item())
                .with_slot(1, self.2.as_item())
                .with_slot(
                    25,
                    TemplateItem::block_tag(
                        "False",
                        "Ignore Y-Axis",
                        "FilterDistance",
                        BlockType::SelectObject,
                    ),
                )
                .with_slot(
                    26,
                    TemplateItem::block_tag(
                        "Nearest",
                        "Compare Mode",
                        "FilterDistance",
                        BlockType::SelectObject,
                    ),
                ),
        ))
    }
}
