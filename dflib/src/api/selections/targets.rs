use crate::api::entity::Entity;
use crate::api::items::number::Number;
use crate::api::items::VarItem;
use crate::api::player::Player;
use crate::api::push_block;
use crate::api::selections::Selection;
use crate::core::args::{ChestArgs, TemplateItem};
use crate::core::template::{BlockType, TemplateBlock};
use std::ops::Deref;

macro_rules! impl_deref_for_sel {
    ($($name:ident),*) => {
        $(
            impl<C: Selection<Base = C>> Deref for $name<C> {
                type Target = <Self as Selection>::Base;

                fn deref(&self) -> &Self::Target {
                    self.selection_mechanism();
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
pub struct EventDefault<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventDefault<C> {
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
pub struct EventKiller<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventKiller<C> {
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
pub struct EventDamager<C: Selection>(pub(crate) C);
impl<C: Selection> Selection for EventDamager<C> {
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
pub struct EventVictim<C: Selection>(pub(crate) C);
impl<C: Selection> Selection for EventVictim<C> {
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
pub struct EventShooter<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventShooter<C> {
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
pub struct EventProjectile<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventProjectile<C> {
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
