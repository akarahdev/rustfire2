use crate::api::items::item::Item;
use crate::api::items::number::Number;
use crate::api::items::string::String;
use crate::api::items::vec::Vector;
use crate::api::items::{set_variable, TypedVarItem, VarItem};
use crate::core::args::{ParticleCluster, ParticleData, TemplateItem as DFItem};

#[derive(Debug, Clone, Copy)]
pub struct Particle(pub(crate) DFItem);

impl VarItem for Particle {
    fn as_item(&self) -> DFItem {
        self.0.clone()
    }

    fn from_item(item: DFItem) -> Self {
        Particle(item)
    }

    fn default() -> Self {
        Particle::new("Cloud")
    }
}

set_variable! {
    self impl Particle; fn (with_amount => "SetParticleAmount") -> Particle;
    arg amount: Number;
}

set_variable! {
    self impl Particle; fn (with_spread => "SetParticleSprd") -> Particle;
    arg horizontal: Number;
    arg vertical: Number;
}

set_variable! {
    self impl Particle; fn (with_size => "SetParticleSize") -> Particle;
    arg size: Number;
}

set_variable! {
    self impl Particle; fn (with_material => "SetParticleMat") -> Particle;
    arg material: Item;
}

set_variable! {
    self impl Particle; fn (with_hex_color => "SetParticleColor") -> Particle;
    arg color: String;
}

set_variable! {
    self impl Particle; fn (with_opacity => "SetParticleOpac") -> Particle;
    arg opacity: Number;
}

set_variable! {
    self impl Particle; fn (with_motion => "SetParticleMotion") -> Particle;
    arg motion: Vector;
}

set_variable! {
    self impl Particle; fn (with_roll => "SetParticleRoll") -> Particle;
    arg roll: Number;
}

set_variable! {
    self impl Particle; fn (with_hex_fade_color => "SetParticleFade") -> Particle;
    arg color: String;
}

impl Particle {
    pub fn new(name: &'static str) -> Particle {
        Particle(DFItem::Particle {
            data: ParticleData {
                particle: name,
                cluster: ParticleCluster {
                    amount: 0,
                    horizontal: 0.0,
                    vertical: 0.0,
                },
            },
        })
    }
}
