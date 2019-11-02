use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::*;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (movable, mut locals, time): Self::SystemData) {

        for (mover, local) in (&movable, &mut locals).join() {
            local.prepend_translation(mover.vel.translation() * time.delta_seconds());
            let axis = mover.vel.rotation().axis();
            if let Some(a) = axis {
                local.prepend_rotation(a, mover.vel.rotation().angle() * time.delta_seconds());
            }
        }
    }
}