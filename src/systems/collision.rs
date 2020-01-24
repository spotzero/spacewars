use amethyst::{
    core::math::Point3,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::Entities,
    renderer::debug_drawing::DebugLinesComponent,
    renderer::palette::Srgba,
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
    );

    fn run(
        &mut self,
        (entities, movables, transforms, colliables, mut collision_events): Self::SystemData,
    ) {
        for (entity1, transform1, movable1, colliable1) in
            (&entities, &transforms, &movables, &colliables).join()
        {
            let mut skip = true;
            for (entity2, transform2, movable2, colliable2) in
                (&entities, &transforms, &movables, &colliables).join()
            {
                if entity1 == entity2 {
                    skip = false;
                    continue;
                }

                if skip {
                    continue;
                }

                let radius = colliable1.radius + colliable2.radius;
                let distance_vec = transform1.translation() - transform2.translation();
                if distance_vec.norm() < radius {
                    collision_events.add_collision(
                        &entity1,
                        &transform1,
                        &movable1,
                        &colliable1,
                        &entity2,
                        &transform2,
                        &movable2,
                        &colliable2,
                    );

                    // If a collision member is debris, delete it.
                    if colliable1.kind == collidable_types::DEBRIS {
                        let _ = entities.delete(entity1);
                    }

                    if colliable2.kind == collidable_types::DEBRIS {
                        let _ = entities.delete(entity2);
                    }
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct DebugCollisionSystem;

impl<'s> System<'s> for DebugCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (transforms, colliables, mut debug_lines): Self::SystemData) {
        for (transform, colliable, debug_line) in
            (&transforms, &colliables, &mut debug_lines).join()
        {
            debug_line.clear();
            let t = transform.translation();
            debug_line.add_circle_2d(
                Point3::new(t[0], t[1], t[2]),
                colliable.radius,
                16,
                Srgba::new(1.0, 0.0, 0.0, 1.0),
            );
        }
    }
}
