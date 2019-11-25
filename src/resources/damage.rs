use amethyst::ecs::Entity;

pub struct Damage {
    target: Entity,
    amount: f32,
    kind: DamageKind,
}

pub enum DamageKind {
    Explosion,
}

pub struct DamageEvents {
    events: Vec<Damage>,
}
