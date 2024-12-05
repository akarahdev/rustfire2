use std::ops::Deref;
use crate::api::push_block;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use crate::entity::Entity;
use crate::impl_deref_for_sel;
use crate::player::Player;
use crate::selections::{ActionTarget, FilterRandomly, Selection};

#[derive(Clone, Debug)]
pub struct AllOf<C: ActionTarget>(pub(crate) C);

impl Deref for AllOf<Player> {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        self.selection_mechanism();
        &Player
    }
}

impl Deref for AllOf<Entity> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.selection_mechanism();
        &Entity
    }
}

impl Selection for AllOf<Player> {
    type Base = Player;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "AllPlayers".to_string(),
            ChestArgs::new(),
        ));
    }
}

impl Selection for AllOf<Entity> {
    type Base = Entity;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "AllEntities".to_string(),
            ChestArgs::new(),
        ));
    }
}

impl AllOf<Player> {
    pub fn player() -> Self {
        AllOf(Player)
    }
}

impl AllOf<Entity> {
    pub fn entity() -> Self {
        AllOf(Entity)
    }
}
