use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::*;
use crate::ARENA_HEIGHT;
use crate::ARENA_WIDTH;

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
            local.prepend_translation(mover.velocity * time.delta_seconds());
            if mover.angular_velocity != 0.0 {
                local.rotate_2d(mover.angular_velocity * time.delta_seconds());
            }

            if local.translation().x < 0.0 {
                local.set_translation_x(ARENA_WIDTH + local.translation().x);
            } else if local.translation().x > ARENA_WIDTH {
                local.set_translation_x(local.translation().x - ARENA_WIDTH);
            }

            if local.translation().y < 0.0 {
                local.set_translation_y(ARENA_HEIGHT + local.translation().y);
            } else if local.translation().y > ARENA_HEIGHT {
                local.set_translation_y(local.translation().y - ARENA_HEIGHT);
            }

        }
    }
}