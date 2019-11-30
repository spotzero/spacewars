use amethyst::ecs::Entity;
use amethyst::core::math::Vector3;

pub struct Collision {
    pub target: Entity,
    pub kind: CollisionKind,
    pub direction: Vector3<f32>,
    pub force: f32,
}

pub enum CollisionKind {
    GravityWell,
    Player,
    Explosion,
    Debris,
    Torpedo,
}

pub struct CollisionEvents {
    pub events: Vec<Collision>,
}

impl CollisionEvents {
    pub fn add_collision(collision_events: &mut CollisionEvents, ) {

    }
}