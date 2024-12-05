use crate::api::entity::Entity;
use crate::api::player::Player;
use crate::api::push_block;
use crate::api::selections::Selection;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use crate::selections::ActionTarget;
use std::ops::Deref;

#[macro_export]
macro_rules! impl_deref_for_sel {
    ($($name:ident),*) => {
        $(
            impl<C> std::ops::Deref for $name<C>
                where C: ActionTarget {
                type Target = C;

                fn deref(&self) -> &Self::Target {
                    self.clone().selection_mechanism();
                    &self.0
                }
            }
        )*
    };
}

impl_deref_for_sel!(
    EventDefault,
    EventKiller,
    EventDamager,
    EventVictim,
    EventShooter,
    EventProjectile
);

#[derive(Clone, Debug)]
pub struct EventDefault<C: ActionTarget>(pub(crate) C);

impl<C: ActionTarget> Selection for EventDefault<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Default",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}

impl EventDefault<Player> {
    pub fn player() -> Self {
        EventDefault(Player)
    }
}

impl EventDefault<Entity> {
    pub fn entity() -> Self {
        EventDefault(Entity)
    }
}

#[derive(Clone, Debug)]
pub struct EventKiller<C: ActionTarget>(pub(crate) C);

impl<C: ActionTarget> Selection for EventKiller<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Killer",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventDamager<C: ActionTarget>(pub(crate) C);
impl<C: ActionTarget> Selection for EventDamager<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Damager",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventVictim<C: ActionTarget>(pub(crate) C);
impl<C: ActionTarget> Selection for EventVictim<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Victim",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventShooter<C: ActionTarget>(pub(crate) C);

impl<C: ActionTarget> Selection for EventShooter<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Shooter",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventProjectile<C: ActionTarget>(pub(crate) C);

impl<C: ActionTarget> Selection for EventProjectile<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new().with_slot(
                26,
                TemplateItem::block_tag(
                    "Projectile",
                    "Event Target",
                    "EventTarget",
                    BlockType::SelectObject,
                ),
            ),
        ));
        self.0.selection_mechanism()
    }
}
