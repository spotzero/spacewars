use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::*;

/*
// ParticleSystem controls the lifetime and fade of emitted particles.
#[derive(SystemDesc)]
pub struct ParticleSystem;

// EngineParticleSystem creates particles when an engine is thrusting, in the reverse direction of thrust.
#[derive(SystemDesc)]
pub struct EngineParticleSystem;

impl<'s> System<'s> for ParticleSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Lifetime>,
    );
}


impl<'s> System<'s> for EngineParticleSystem {
    type SystemData = (
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
    );

}

*/