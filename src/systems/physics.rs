use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, WriteStorage},
};

use crate::components::*;
use crate::resources::Game;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(&mut self, (mut movable, mut transforms, time, game): Self::SystemData) {
        if !game.is_playing() {
            return;
        }

        let gravitywell = Vector3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

        for (mover, transform) in (&mut movable, &mut transforms).join() {
            if !mover.apply_physics {
                continue;
            }

            transform.prepend_translation(mover.velocity * time.delta_seconds());
            if mover.angular_velocity != 0.0 {
                transform.rotate_2d(mover.angular_velocity * time.delta_seconds());
            }

            if transform.translation().x < 0.0 {
                transform.set_translation_x(ARENA_WIDTH + transform.translation().x);
            } else if transform.translation().x > ARENA_WIDTH {
                transform.set_translation_x(transform.translation().x - ARENA_WIDTH);
            }

            if transform.translation().y < 0.0 {
                transform.set_translation_y(ARENA_HEIGHT + transform.translation().y);
            } else if transform.translation().y > ARENA_HEIGHT {
                transform.set_translation_y(transform.translation().y - ARENA_HEIGHT);
            }

            // Apply gravity.
            let dir = gravitywell - transform.translation();
            let dis = dir.magnitude();
            if dis > 20. {
                let gravity = ((5000000.0 * dir.normalize()) / (dis * dis)) * time.delta_seconds();
                mover.velocity += gravity;
            }
        }
    }
}
