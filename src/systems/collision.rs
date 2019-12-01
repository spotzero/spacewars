use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::Entities,
};

use crate::components::*;
use crate::resources::*;

/// This system finds all the collisions and queues events for them.
#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        WriteExpect<'s, CollisionEvents>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, movable, transforms, colliables, mut collision_events, time): Self::SystemData) {
        for (entity1, transform1, colliable1) in (&entities, &transforms, &colliables).join() {
            let mut skip = true;
            for (entity2, transform2, collisble2) in (&entities, &transforms, &colliables).join() {
                if entity1 == entity2 {
                    skip = false;
                    continue;
                }

                if (skip) {
                  continue;
                }

                let radius = colliable1.radius + collisble2.radius;
                let distance_vec = transform1.translation() - transform2.translation();
                if (distance_vec.norm() < radius) {
                    collision_events.add_collision(
                        &entity1, &transform1, &colliable1,
                        &entity2, &transform2, &collisble2
                    );
                }
            }
        }
    }
}

/*
#[derive(SystemDesc)]
pub struct ExplosionCollisionResponseSystem;

#[derive(SystemDesc)]
pub struct GravityWellCollisionResponseSystem;

#[derive(SystemDesc)]
pub struct PlayerCollisionResponseSystem;
*/
