use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    ecs::Entities,
};

use crate::components::*;

#[derive(SystemDesc)]
pub struct GravitywellCollisionSystem;

impl<'s> System<'s> for GravitywellCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, movable, transforms, colliables, time): Self::SystemData) {

        /*
        for (entity1, transform2, colliable2) in (&entities, &transforms, &colliables).join() {
            for (entity2, transform2, collisble2) in (&entities, &transforms, &colliables).join() {
                if entity == entity1 {
                    continue;
                }

            }
        }
        */
    }
}
