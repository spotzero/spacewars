use amethyst::ecs::Entity;

pub struct Collision {
    pub target: Entity,
    pub kind: CollisionKind,
}

pub enum CollisionKind {
    Explosion,
    GravityWell,
    Ship,
    Debris,
    Torpedo,
}

pub struct CollisionEvents {
    events: Vec<Collision>,
}
