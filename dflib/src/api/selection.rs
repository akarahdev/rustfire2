use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use crate::api::push_block;
use crate::codetemplate::args::{ChestArgs, Item};
use crate::codetemplate::template::{BlockType, TemplateBlock};

pub trait Selection: Clone + Debug {
    type Base;

    fn selection_mechanism(&self) -> Self::Base;
}

#[derive(Clone, Debug)]
enum Target {
    Default,
    Killer,
    Damager,
    Victim,
    Shooter,
    Projectile,
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Target::Default => "Default",
            Target::Killer => "Killer",
            Target::Damager => "Damager",
            Target::Victim => "Victim",
            Target::Shooter => "Shooter",
            Target::Projectile => "Projectile",
        })?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct EventDefault<C: Selection>(pub(crate) C);
impl<C: Selection<Base = C>> Deref for EventDefault<C> {
    type Target = <Self as Selection>::Base;

    fn deref(&self) -> &Self::Target {
        self.selection_mechanism();
        &self.0
    }
}
impl<C: Selection> Selection for EventDefault<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Default", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventKiller<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventKiller<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Killer", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventDamager<C: Selection>(pub(crate) C);
impl<C: Selection> Selection for EventDamager<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Damager", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventVictim<C: Selection>(pub(crate) C);
impl<C: Selection> Selection for EventVictim<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Victim", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventShooter<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventShooter<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Shooter", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}

#[derive(Clone, Debug)]
pub struct EventProjectile<C: Selection>(pub(crate) C);

impl<C: Selection> Selection for EventProjectile<C> {
    type Base = <C as Selection>::Base;

    fn selection_mechanism(&self) -> Self::Base {
        push_block(TemplateBlock::select_object(
            "EventTarget".to_string(),
            ChestArgs::new()
                .with_slot(26, Item::block_tag("Projectile", "Event Target",
                                               "EventTarget", BlockType::SelectObject)),
        ));
        self.0.selection_mechanism()
    }
}