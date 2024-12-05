use crate::api::config::PlotRank;
use crate::api::items::loc::Location;
use crate::api::items::number::Number;
use crate::api::items::particle::Particle;
use crate::api::items::VarItem;
use crate::api::player::player_action;

player_action! {
    fn particle => "Particle";

    arg effect: Particle;
    arg location: Location;
}

player_action! {
    fn particle_line => "ParticleLine";

    arg effect: Particle;
    arg start: Location;
    arg end: Location;
}

player_action! {
    requires PlotRank::Noble;

    fn particle_circle => "ParticleCircle";

    arg effect: Particle;
    arg loc: Location;
    arg diameter: Number;
}

player_action! {
    requires PlotRank::Mythic;

    fn particle_circle_animated => "ParticleCircleA";

    arg effect: Particle;
    arg loc: Location;
    arg diameter: Number;
    arg duration: Number;
}

player_action! {
    requires PlotRank::Noble;

    fn particle_cuboid => "ParticleCuboid";

    arg effect: Particle;
    arg corner1: Location;
    arg corner2: Location;
    arg spacing: Number;

    tag "Fill Type" => "Wireframe";
}

player_action! {
    requires PlotRank::Mythic;

    fn particle_cuboid_animated => "ParticleCuboidA";

    arg effect: Particle;
    arg corner1: Location;
    arg corner2: Location;
    arg spacing: Number;

    tag "Fill Type" => "Wireframe";
}
