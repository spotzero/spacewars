use amethyst::ecs::Entity;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use crate::resources::*;
use crate::components::*;

pub struct TorpedoCollision {
    pub torpedo: u32,
    pub collided: u32,
}

/// Ships, Debris colliding with each other or with explosions.
///
/// Anything with mass colliding, or colliding with an explosion takes damage and gets a push away.
pub struct ForceCollision {
    pub target: u32,
    pub damage: f32,
    pub force: Vector3<f32>,
}

pub struct GravityWellCollision {

}

#[derive(Default)]
pub struct CollisionEvents {
    pub torpedo_collisions: Vec<TorpedoCollision>,
    pub player_collisions: Vec<ForceCollision>,
}

impl CollisionEvents {
    pub fn add_collision(
        &mut self,
        entity1: &Entity,
        transform1: &Transform,
        colliable1: &Collidable,
        entity2: &Entity,
        transform2: &Transform,
        colliable2: &Collidable
    ) {
        if colliable1.kind == CollidableTypes::TORPEDO {
            self.torpedo_collisions.push(TorpedoCollision {
                torpedo: entity1.id(),
                collided: entity2.id(),
            });
        }

        if colliable2.kind == CollidableTypes::TORPEDO {
            self.torpedo_collisions.push(TorpedoCollision {
                torpedo: entity2.id(),
                collided: entity1.id(),
            });
        }
    }
}