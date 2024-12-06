use std::marker::PhantomData;
use std::ops::Deref;
use crate::api::push_block;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use crate::entity::Entity;
use crate::items::{List, Number, String, VarItem};
use crate::player::Player;
use crate::selections::Selection;

#[derive(Clone, Debug)]
pub struct Stored<C: Selection>(pub(crate) List<String>, pub(crate) PhantomData<C>);

impl Deref for Stored<Player> {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        self.selection_mechanism();
        &Player
    }
}

impl Deref for Stored<Entity> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.selection_mechanism();
        &Entity
    }
}

impl Selection for Stored<Player> {
    type Base = Player;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "PlayerName".to_string(),
            ChestArgs::new()
                .with_slot(0, self.0.as_item()),
        ))
    }
}

impl Selection for Stored<Entity> {
    type Base = Player;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EntityName".to_string(),
            ChestArgs::new()
                .with_slot(0, self.0.as_item())
                .with_slot(26, TemplateItem::block_tag(
                    "True", "Ignore Formatting",
                        "EntityName", BlockType::SelectObject
                )),
        ))
    }
}