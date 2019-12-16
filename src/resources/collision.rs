use amethyst::ecs::Entity;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;

use crate::resources::*;
use crate::components::*;

pub struct TorpedoCollision {
    pub torpedo: u32,
    pub collided: u32,
    pub direction: Vector3<f32>,
}

/// Ships, Debris colliding with each other or with explosions.
///
/// Anything with mass colliding, or colliding with an explosion takes damage and gets a push away.
pub struct ForceCollision {
    pub target: u32,
    pub damage: f32,
    pub force: Vector3<f32>,
}

pub struct ExplosionCollision {
    pub explosion: u32,
    pub player: u32,
    pub distance: f32,
}

pub struct GravityWellCollision {
    pub player: u32,
}

#[derive(Default)]
pub struct CollisionEvents {
    pub torpedo_collisions: Vec<TorpedoCollision>,
    pub player_collisions: Vec<ForceCollision>,
    pub explosion_collisions: Vec<ExplosionCollision>,
    pub gravity_well_collision: Vec<GravityWellCollision>,
}

impl CollisionEvents {
    pub fn add_collision(
        &mut self,
        entity1: &Entity,
        transform1: &Transform,
        movable1: &Movable,
        colliable1: &Collidable,
        entity2: &Entity,
        transform2: &Transform,
        movable2: &Movable,
        colliable2: &Collidable
    ) {
        let dir1 = transform1.translation() - transform2.translation();
        let dir2 = transform1.translation() - transform2.translation();

        // Collision between anything and torpedo.
        if colliable1.kind == collidable_types::TORPEDO {
            self.torpedo_collisions.push(TorpedoCollision {
                direction: unit_vector(&dir1),
                torpedo: entity1.id(),
                collided: entity2.id(),
            });
            return;
        }

        if colliable2.kind == collidable_types::TORPEDO {
            self.torpedo_collisions.push(TorpedoCollision {
                direction: unit_vector(&dir2),
                torpedo: entity2.id(),
                collided: entity1.id(),
            });
            return;
        }

        // Collision between player and explosions.
        if (
            colliable1.kind == collidable_types::PLAYER
            && colliable2.kind == collidable_types::EXPLOSION
        ) || (
            colliable2.kind == collidable_types::PLAYER
            && colliable1.kind == collidable_types::EXPLOSION
        ) {
            self.explosion_collisions.push(ExplosionCollision {
                explosion: if colliable1.kind == collidable_types::PLAYER {
                    entity2.id()
                } else {
                    entity1.id()
                },
                player: if colliable1.kind == collidable_types::PLAYER {
                    entity1.id()
                } else {
                    entity2.id()
                },
                distance: (transform1.translation() - transform2.translation()).norm(),
            });
            return;
        }

        // Collision between player and the grav well.
        if (
            colliable1.kind == collidable_types::PLAYER
            && colliable2.kind == collidable_types::GRAVITYWELL
        ) || (
            colliable2.kind == collidable_types::PLAYER
            && colliable1.kind == collidable_types::GRAVITYWELL
        ) {
            self.gravity_well_collision.push(GravityWellCollision {
                player: if colliable1.kind == collidable_types::PLAYER {
                    entity1.id()
                } else {
                    entity2.id()
                }
            });
            return;
        }
    }
}
